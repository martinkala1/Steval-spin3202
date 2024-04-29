#![no_std]
#![no_main]

use core::{cell::RefCell, panic::PanicInfo};

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
// use panic_halt as _;
use six_step::Motor;
use stm32f0::stm32f0x1::{self, interrupt, GPIOB, GPIOF, NVIC, TIM2};

// use crate::setup::{configure_gpiof, configure_sysclk_pll};
use system_clocks::{configure_sysclk_pll, delay_ms};
use gpio::{configure_gpiof, configure_gpiob};
use timers::{configure_tim1, configure_tim2, Pwm, PwmChannel};

mod error_type;
mod gpio;
mod system_clocks;
mod timers;
mod six_step;


static GGPIOF: Mutex<RefCell<Option<GPIOF>>> = Mutex::new(RefCell::new(None));
static GTIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));

#[panic_handler]
unsafe fn panic_handler(info: &PanicInfo) -> ! {
    hprintln!("ERROR! {:?}", info).unwrap();
    loop {}
}

#[entry]
fn main() -> ! {
    let peripherals = stm32f0x1::Peripherals::take().unwrap();
    configure_sysclk_pll(&peripherals);
    hprintln!("Clock ready.").unwrap();
    configure_tim1(&peripherals);
    hprintln!("TIM1 ready.").unwrap();
    configure_tim2(&peripherals);
    hprintln!("TIM2 ready.").unwrap();
    configure_gpiof(&peripherals);
    hprintln!("GPIOF ready.").unwrap();
    configure_gpiob(&peripherals);
    hprintln!("GPIOB ready.").unwrap();

    free(|cs| {
        GGPIOF.borrow(cs).replace(Some(peripherals.GPIOF));
        GTIM2.borrow(cs).replace(Some(peripherals.TIM2));
    });

    unsafe {
        NVIC::unmask(interrupt::TIM2);
    }

    // let delay = 10;
    // let mut pwm = Pwm{tim: &peripherals.TIM1};
    // let motor = Motor::new(&mut pwm);
    hprintln!("Initialization complete!").unwrap();


    // NVIC::pend(interrupt::TIM2);
    // delay_ms(2000);
    // // NVIC::pend(interrupt::TIM2);
    loop {

        // pwm.pwm_stop(PwmChannel::Channel1);
        // pwm.pwm_start(PwmChannel::Channel2);
        // delay_ms(delay);

        // peripherals.GPIOB.odr.write(|w| w.odr15().clear_bit().odr13().set_bit()) ;
        // delay_ms(delay);

        // pwm.pwm_stop(PwmChannel::Channel2);
        // pwm.pwm_start(PwmChannel::Channel3);
        // delay_ms(delay);

        // peripherals.GPIOB.odr.write(|w| w.odr13().clear_bit().odr14().set_bit()) ;
        // delay_ms(delay);

        // pwm.pwm_stop(PwmChannel::Channel3);
        // pwm.pwm_start(PwmChannel::Channel1);
        // delay_ms(delay);

        // peripherals.GPIOB.odr.write(|w| w.odr14().clear_bit().odr15().set_bit()) ;
        // delay_ms(delay);

        // if peripherals.GPIOF.idr.read().idr1().bit_is_set() {
        //     peripherals.GPIOF.odr.modify(|r, w| w.odr0().bit(!r.odr0().bit()));
        // }

        // peripherals.GPIOF.odr.modify(|r, w| w.odr0().bit(!r.odr0().bit()));
        // delay_ms(1000);
        // if peripherals.TIM2.sr.read().uif().bit_is_set() {
        //     peripherals.TIM2.sr.write(|w| w.uif().clear_bit());
        //     peripherals.GPIOF.odr.modify(|r, w| w.odr0().bit(!r.odr0().bit()));
        // }
        // delay_ms(1000);
    }
}

#[interrupt]
fn TIM2() {
    static mut GPIOF_WRAPPER: Option<GPIOF> = None;
    static mut TIM_WRAPPER: Option<TIM2> = None;
    let tim = TIM_WRAPPER.get_or_insert_with(|| {
        free(|cs| {
            GTIM2.borrow(cs).replace(None).unwrap()
        })
    });

    let gpiof = GPIOF_WRAPPER.get_or_insert_with(|| {
        free(|cs| {
            GGPIOF.borrow(cs).replace(None).unwrap()
        })
    });

    tim.sr.modify(|_,w| w.uif().clear_bit());
    gpiof.odr.modify(|r,w| w.odr0().bit(!r.odr0().bit()));

}
