#![no_std]
#![no_main]

use core::ops::DerefMut;
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

use six_step::Motor;
use stm32f0::stm32f0x1::{self, interrupt, tim1::cnt, ADC, GPIOF, NVIC, TIM2};
use system_clocks::{configure_sysclk_pll, delay_ms};
use gpio::configure_gpio;
use timers::{configure_tim1, configure_tim2, Pwm};
use adc::configure_adc;

mod error_type;
mod gpio;
mod system_clocks;
mod timers;
mod six_step;
mod adc;


static GGPIOF: Mutex<RefCell<Option<GPIOF>>> = Mutex::new(RefCell::new(None));
static GTIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));
static GADC: Mutex<RefCell<Option<ADC>>> = Mutex::new(RefCell::new(None));

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

    free(|cs| {
        GTIM2.borrow(cs).replace(Some(tim2));
        GGPIOF.borrow(cs).replace(Some(gpiof));
        GADC.borrow(cs).replace(Some(adc));
    });

    unsafe {
        // NVIC::unmask(interrupt::TIM2);
        NVIC::unmask(interrupt::ADC_COMP);
    }

    let mut pwm = Pwm{tim: &peripherals.TIM1, gpio: &peripherals.GPIOB};
    let mut motor = Motor::new(&mut pwm);
    hprintln!("Initialization complete!").unwrap();


    motor.start(true);
    motor.stop();
    loop {
        // if adc.isr.read().eoc().is_complete() {
        //     gpiof.odr.modify(|r,w| w.odr0().bit(!r.odr0().bit()));
        //     delay_ms(500);
        // }
    }
}

#[interrupt]
fn ADC_COMP() {
    free(|cs| {
        let mut gpio_ref = GGPIOF.borrow(cs).borrow_mut();
        let mut adc_ref = GADC.borrow(cs).borrow_mut();
        if let (Some(ref mut gpio), Some(ref mut adc)) = (gpio_ref.deref_mut(), adc_ref.deref_mut()) {
            gpio.odr.modify(|r,w| w.odr0().bit(!r.odr0().bit()));
            adc.isr.modify(|_, w| w.eoseq().clear().eoc().clear());
        }
    });
} 

// #[interrupt]
// fn TIM2() {
//     free(|cs| {
//         let mut gpio_ref = GGPIOF.borrow(cs).borrow_mut();
//         let mut tim_ref = GTIM2.borrow(cs).borrow_mut();
//         if let (Some(ref mut gpio), Some(ref mut tim)) = (gpio_ref.deref_mut(), tim_ref.deref_mut()) {
//             gpio.odr.modify(|r,w| w.odr0().bit(!r.odr0().bit()));
//             tim.sr.modify(|_,w| w.uif().clear_bit());
//         }
//     });
// }
