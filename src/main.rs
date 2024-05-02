#![no_std]
#![no_main]

use core::ops::DerefMut;
use core::cell::RefCell;
// use rclite::Rc;

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

use six_step::Motor;
use stm32f0::stm32f0x1::{self, interrupt, ADC, GPIOF, NVIC, TIM2};
use system_clocks::{configure_sysclk_pll, delay_ms, delay_us};
use gpio::configure_gpio;
use timers::{configure_tim1, configure_tim2, Pwm};
use adc::configure_adc;

mod gpio;
mod system_clocks;
mod timers;
mod six_step;
mod adc;


// static GGPIOF: Mutex<RefCell<Option<GPIOF>>> = Mutex::new(RefCell::new(None));
static GTIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));
static GADC: Mutex<RefCell<Option<ADC>>> = Mutex::new(RefCell::new(None));
static MOTOR: Mutex<RefCell<Option<Motor>>> = Mutex::new(RefCell::new(None));
static LSB: u32 = 800; // 0.8mV

#[entry]
fn main() -> ! {
    let peripherals = stm32f0x1::Peripherals::take().unwrap();
    configure_sysclk_pll(&peripherals);
    hprintln!("Clock ready.").unwrap();
    configure_tim1(&peripherals);
    hprintln!("TIM1 ready.").unwrap();
    configure_tim2(&peripherals);
    hprintln!("TIM2 ready.").unwrap();
    configure_gpio(&peripherals);
    hprintln!("GPIO ready.").unwrap();
    configure_adc(&peripherals);
    hprintln!("ADC ready.").unwrap();

    let gpiof = peripherals.GPIOF;
    let tim2 = peripherals.TIM2;
    let adc = peripherals.ADC;
    let pwm = Pwm{tim: peripherals.TIM1, gpio: peripherals.GPIOB};
    let mut motor = Motor::new(pwm);
    free(|cs| {
        GTIM2.borrow(cs).replace(Some(tim2));
        // GGPIOF.borrow(cs).replace(Some(gpiof));
        GADC.borrow(cs).replace(Some(adc));
        // MOTOR.borrow(cs).replace(Some(motor));
    });

    motor.engage_step();
    unsafe {
        NVIC::unmask(interrupt::ADC_COMP);
    }

    
    hprintln!("Initialization complete!").unwrap();

    // motor.start(true, &tim3);

    loop {
        // gpiof.odr.modify(|r,w| w.odr0().bit(!r.odr0().bit()));
        // delay_us(1_000_000);
    }
}

#[interrupt]
fn ADC_COMP() {
    static mut CNT: u32 = 0;
    static mut ADC_WRAPPER: Option<ADC> = None;
    let adc = ADC_WRAPPER.get_or_insert_with(|| {
        free(|cs|{
            GADC.borrow(cs).replace(None).unwrap()
        })
    });

    hprintln!("{}mV",(adc.dr.read().data().bits() as u32 * LSB) / 1000).unwrap();
    adc.isr.modify(|_, w| w.eoc().clear());
    *CNT+=1;
} 
