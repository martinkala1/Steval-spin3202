#![no_std]
#![no_main]

use core::{borrow::BorrowMut, ops::DerefMut};
use core::cell::RefCell;
// use rclite::Rc;

use cortex_m::interrupt::{free, Mutex};
use cortex_m::peripheral;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

use six_step::Motor;
use stm32f0::stm32f0x1::{gpiob, CorePeripherals};
use stm32f0::stm32f0x1::{self, interrupt, tim1::CNT, ADC, GPIOF, NVIC, TIM2};
use system_clocks::{configure_sysclk_pll, delay_ms, delay_us};
use gpio::configure_gpio;
use timers::{configure_tim1, Pwm};
use adc::configure_adc;
use usart::configure_usart;

mod gpio;
mod system_clocks;
mod timers;
mod six_step;
mod adc;
mod usart;

static GGPIOF: Mutex<RefCell<Option<GPIOF>>> = Mutex::new(RefCell::new(None));
static GTIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));
static GADC: Mutex<RefCell<Option<ADC>>> = Mutex::new(RefCell::new(None));
static GMOTOR: Mutex<RefCell<Option<Motor>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = stm32f0x1::Peripherals::take().unwrap();
    configure_sysclk_pll(&peripherals);
    hprintln!("Clock ready.").unwrap();
    configure_tim1(&peripherals);
    hprintln!("TIM1 ready.").unwrap();
    // configure_tim2(&peripherals);
    // hprintln!("TIM2 ready.").unwrap();
    configure_gpio(&peripherals);
    hprintln!("GPIO ready.").unwrap();
    configure_adc(&peripherals);
    hprintln!("ADC ready.").unwrap();
    configure_usart(&peripherals);
    hprintln!("USART ready.").unwrap();

    let gpiof = peripherals.GPIOF;
    let usart = peripherals.USART1;
    let tim2 = peripherals.TIM2;
    let adc = peripherals.ADC;
    let pwm = Pwm{tim: peripherals.TIM1, gpio: peripherals.GPIOB};
    let motor = Motor::new(pwm);
    free(|cs| {
        GGPIOF.borrow(cs).replace(Some(gpiof));
        GTIM2.borrow(cs).replace(Some(tim2));
        GADC.borrow(cs).replace(Some(adc));
        GMOTOR.borrow(cs).replace(Some(motor))
    });
    unsafe {
        NVIC::unmask(interrupt::ADC_COMP);
    }
    hprintln!("Initialization complete!").unwrap();
    let mut received_char: u8;
    loop {
        // if usart.isr.read().rxne().bit_is_set() {
        //     received_char = usart.rdr.read().rdr().bits() as u8;
        //     hprintln!("Received char: {}", received_char as char).unwrap();
        // }
        // gpiof.odr.modify(|r,w| w.odr0().bit(!r.odr0().bit()));
        // delay_us(1_000_000);
    }
}

fn reconfigure_adc_channel(adc: &mut ADC, current_step: usize) {
    match current_step { // change channel
        0 | 3 => { adc.chselr.write(|w| w.chsel2().selected()); }
        1 | 4 => { adc.chselr.write(|w| w.chsel1().selected()); }
        2 | 5 => { adc.chselr.write(|w| w.chsel0().selected()); }
        _ => panic!("Unknown step.")
    };
}

static V_BUS_HALF: i32 = 1800;

///called every 50us
#[interrupt]
fn ADC_COMP() {
    static mut CALL_CNT: u32 = 0;
    static mut TIMER: u32 = 0;
    static mut COMM_DELAY: u32 = 0;
    static mut STATE: u8 = 0;
    static mut ADC_WRAPPER: Option<ADC> = None;
    static mut MOTOR_WRAPPER: Option<Motor> = None;
    static mut GPIOF_WRAPPER: Option<GPIOF> = None;
    static mut PREV_BEMF: i32 = 0;
    static mut DELAY: u32 = 40;
    let gpiof = GPIOF_WRAPPER.get_or_insert_with(|| {
        free(|cs|{
            GGPIOF.borrow(cs).replace(None).unwrap()
        })
    });
    let adc = ADC_WRAPPER.get_or_insert_with(|| {
        free(|cs|{
            GADC.borrow(cs).replace(None).unwrap()
        })
    });
    let motor = MOTOR_WRAPPER.get_or_insert_with(|| {
        free(|cs|{
            GMOTOR.borrow(cs).replace(None).unwrap()
        })
    });
    match STATE {
        0 => {
            if gpiof.idr.read().idr1().bit_is_clear() {
                *STATE = 102;
                *TIMER = 0;
                *COMM_DELAY = 0;
                *PREV_BEMF = 0;
                *DELAY = 40;
                *CALL_CNT = 0;
            }
            if (*CALL_CNT > 1_000_000) {
                *CALL_CNT = 0;
            }
        }
        102 => {
            if *CALL_CNT > 1_000 {
                motor.engage_step();
                *CALL_CNT = 0;
                *STATE = 1;
            }
        }
        1 => {
            if *CALL_CNT >= *DELAY{
                motor.next_step(true);
                *CALL_CNT = 0;
                *DELAY = *DELAY - 1;
                if *DELAY <= 20 {
                    *STATE = 2;
                }
            }
        }
        2 => {
            if *CALL_CNT > 4 {
                let curr_bemf = ((adc.dr.read().data().bits() * 8) / 10) as i32 - V_BUS_HALF;
                if (*PREV_BEMF) * curr_bemf < 0 {
                    // zero-crossing event found
                    *COMM_DELAY = *CALL_CNT;
                    *PREV_BEMF = 0;
                    *CALL_CNT = 0;
                    *STATE = 10;
                    *TIMER = 0
                }
                *PREV_BEMF = curr_bemf;
            }
            if *CALL_CNT >= *DELAY {
                motor.next_step(true);
                *STATE = 3;
                *TIMER += *CALL_CNT;
                *CALL_CNT = 0;
            }
            if *TIMER >= 20_000 { // time to start feedback-loop elapsed, stop motor
                *STATE = 100;
            }
        }
        3 => {
            reconfigure_adc_channel(adc, motor.actual_step_index);
            *STATE = 2;
        }
        10 => {
            if *CALL_CNT >= *COMM_DELAY {
                *TIMER += *CALL_CNT;
                *CALL_CNT = 0;
                *STATE = 11;
            }    
        }
        11 => {
            if *TIMER > 20_000 && gpiof.idr.read().idr1().bit_is_clear() { // stop motor when button is pressed
                *STATE = 100;
            } else {
                motor.next_step(true);
                *STATE = 12;
            }
        }
        12 => {
            reconfigure_adc_channel(adc, motor.actual_step_index);
            *STATE = 13;
        }
        13 => {
            if *CALL_CNT > 4 {
                let curr_bemf = ((adc.dr.read().data().bits() * 8) / 10) as i32 - V_BUS_HALF;
                if (*PREV_BEMF) * curr_bemf < 0 {
                    // zero-crossing event found
                    *COMM_DELAY = *CALL_CNT;
                    *PREV_BEMF = 0;
                    *TIMER += *CALL_CNT;
                    *CALL_CNT = 0;
                    *STATE = 10;
                    if *TIMER > 1_000_000 {
                        *TIMER = 0;
                    }
                }
                *PREV_BEMF = curr_bemf;
            }
        }
        100 => {
            motor.stop();
            *CALL_CNT = 0;
            *STATE = 101;
        }
        101 => {
            if *CALL_CNT >= 10_000 {
                *CALL_CNT = 0;
                *STATE = 0;
            }
        }
        _ => {}
    }
    adc.isr.modify(|_, w| w.eoc().clear());
    *CALL_CNT+=1;
} 