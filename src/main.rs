#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use panic_halt as _;

use six_step::Motor;
use stm32f0::stm32f0x1::{self, interrupt, ADC, GPIOF, NVIC, TIM2, USART1};
use system_clocks::configure_sysclk_pll;
use gpio::configure_gpio;
use timers::{configure_tim1, Pwm};
use adc::configure_adc;
use uart::{configure_uart, uart_read, uart_read_async, uart_send};

mod gpio;
mod system_clocks;
mod timers;
mod six_step;
mod adc;
mod uart;

static GGPIOF: Mutex<RefCell<Option<GPIOF>>> = Mutex::new(RefCell::new(None));
static GTIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));
static GADC: Mutex<RefCell<Option<ADC>>> = Mutex::new(RefCell::new(None));
static GMOTOR: Mutex<RefCell<Option<Motor>>> = Mutex::new(RefCell::new(None));
static GUART: Mutex<RefCell<Option<USART1>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = stm32f0x1::Peripherals::take().unwrap();
    configure_sysclk_pll(&peripherals);
    configure_tim1(&peripherals);
    configure_gpio(&peripherals);
    configure_adc(&peripherals);
    configure_uart(&peripherals);

    let gpiof = peripherals.GPIOF;
    let uart = peripherals.USART1;
    let tim2 = peripherals.TIM2;
    let adc = peripherals.ADC;
    let pwm = Pwm{tim: peripherals.TIM1, gpio: peripherals.GPIOB};
    let motor = Motor::new(pwm);
    uart_send(&uart, "Configuration completed.\n\r");
    free(|cs| {
        GGPIOF.borrow(cs).replace(Some(gpiof));
        GTIM2.borrow(cs).replace(Some(tim2));
        GADC.borrow(cs).replace(Some(adc));
        GMOTOR.borrow(cs).replace(Some(motor));
        GUART.borrow(cs).replace(Some(uart));
    });
    unsafe {
        NVIC::unmask(interrupt::ADC_COMP);
    }

    loop {}
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
    static mut UART_WRAPPER: Option<USART1> = None;
    static mut PREV_BEMF: i32 = 0;
    static mut DELAY: u32 = 40;
    static mut SPEED: u32 = 50;
    static mut SPEED_CHANGED: bool = false;
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
    let uart1 = UART_WRAPPER.get_or_insert_with(|| {
        free(|cs|{
            GUART.borrow(cs).replace(None).unwrap()
        })
    });
    let motor = MOTOR_WRAPPER.get_or_insert_with(|| {
        free(|cs|{
            GMOTOR.borrow(cs).replace(None).unwrap()
        })
    });
    match STATE {
        0 => {
            if uart1.isr.read().rxne().bit_is_set() { // start if 'g' is received
                if uart_read(uart1) == 'g' {
                    *STATE = 102;
                    motor.set_speed(50);
                    *TIMER = 0;
                    *COMM_DELAY = 0;
                    *PREV_BEMF = 0;
                    *DELAY = 40;
                    *CALL_CNT = 0;
                }
            }
            if gpiof.idr.read().idr1().bit_is_clear() { // start if button is pressed
                *STATE = 102;
                motor.set_speed(50);
                *TIMER = 0;
                *COMM_DELAY = 0;
                *PREV_BEMF = 0;
                *DELAY = 40;
                *CALL_CNT = 0;
            }
            if *CALL_CNT > 1_000_000 { // avoid overflow
                *CALL_CNT = 0;
            }
        }
        102 => {
            if *CALL_CNT > 1_000 { // button debouncing delay
                motor.engage_step();
                *CALL_CNT = 0;
                *STATE = 1;
            }
        }
        1 => {
            if *CALL_CNT >= *DELAY{ // slowly decrease the delay
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
                if (*PREV_BEMF) * curr_bemf < 0 { // zero crossing event detection
                    *COMM_DELAY = *CALL_CNT;
                    *PREV_BEMF = 0;
                    *CALL_CNT = 0;
                    *STATE = 10;
                    *TIMER = 0
                }
                *PREV_BEMF = curr_bemf;
            }
            if *CALL_CNT >= *DELAY { // next step without feedback
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
            reconfigure_adc_channel(adc, motor.actual_step_index); // to measure floating phase
            *STATE = 2;
        }
        10 => {
            if *SPEED_CHANGED == true {
                *SPEED_CHANGED = false;
                motor.set_speed(*SPEED);
            }
            match uart_read_async(uart1) {
                'h' => {
                    if *TIMER > 20_000 { *STATE = 100;}
                }
                'f' => {
                    if *SPEED < 130 {
                        *SPEED += 10;
                        *SPEED_CHANGED = true;
                    }
                }
                's' => {
                    if *SPEED > 30 {
                        *SPEED -= 10;
                        *SPEED_CHANGED = true;
                    }
                }
                _ => {}
            }
            if *CALL_CNT >= *COMM_DELAY { // commutation delay state
                *TIMER += *CALL_CNT;
                *CALL_CNT = 0;
                *STATE = 11;
            }    
        }
        11 => {
            if *TIMER > 20_000 && (gpiof.idr.read().idr1().bit_is_clear() || uart_read_async(uart1) == 'h'){ // stop motor when button is pressed
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
            if *CALL_CNT >= 10_000 { // button debouncing, go to starting state
                *CALL_CNT = 0;
                *STATE = 0;
            }
        }
        _ => {}
    }
    adc.isr.modify(|_, w| w.eoc().clear());
    *CALL_CNT+=1;
} 