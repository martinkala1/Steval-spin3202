#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f0::stm32f0x1;

// use crate::setup::{configure_gpiof, configure_sysclk_pll};
use system_clocks::{configure_sysclk_pll, delay_ms};
use gpio::{configure_gpiof, configure_gpiob};
use timers::{configure_tim1, Pwm, PwmChannel};

mod error_type;
mod gpio;
mod system_clocks;
mod timers;

#[entry]
fn main() -> ! {
    let peripherals = stm32f0x1::Peripherals::take().unwrap();
    let pwm = Pwm{tim: &peripherals.TIM1};
    configure_sysclk_pll(&peripherals);
    configure_tim1(&peripherals);
    configure_gpiof(&peripherals);
    configure_gpiob(&peripherals);

    hprintln!("Kámo, je to tam!").unwrap();

    // pwm.pwm_start(PwmChannel::Channel1);
    // peripherals.GPIOB.odr.write(|w| w.odr15().set_bit());

    // delay_ms(200);
    // pwm.pwm_stop(PwmChannel::Channel1);
    let delay = 10;
    loop {

        pwm.pwm_stop(PwmChannel::Channel1);
        pwm.pwm_start(PwmChannel::Channel2);
        delay_ms(delay);

        peripherals.GPIOB.odr.write(|w| w.odr15().clear_bit().odr13().set_bit()) ;
        delay_ms(delay);

        pwm.pwm_stop(PwmChannel::Channel2);
        pwm.pwm_start(PwmChannel::Channel3);
        delay_ms(delay);

        peripherals.GPIOB.odr.write(|w| w.odr13().clear_bit().odr14().set_bit()) ;
        delay_ms(delay);

        pwm.pwm_stop(PwmChannel::Channel3);
        pwm.pwm_start(PwmChannel::Channel1);
        delay_ms(delay);

        peripherals.GPIOB.odr.write(|w| w.odr14().clear_bit().odr15().set_bit()) ;
        delay_ms(delay);

        // if peripherals.GPIOF.idr.read().idr1().bit_is_set() {
        //     peripherals.GPIOF.odr.modify(|r, w| w.odr0().bit(!r.odr0().bit()));
        // }

        peripherals.GPIOF.odr.modify(|r, w| w.odr0().bit(!r.odr0().bit()));
    }
}
