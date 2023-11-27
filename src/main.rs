#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use setup::configure_tim1;
use stm32f0::stm32f0x1;

use crate::setup::{configure_gpiof, configure_sysclk_pll};

mod error_type;
mod setup;

#[entry]
fn main() -> ! {
    let peripherals = stm32f0x1::Peripherals::take().unwrap();

    configure_sysclk_pll(&peripherals);
    configure_tim1(&peripherals);
    configure_gpiof(&peripherals);

    hprintln!("Kámo, je to tam!").unwrap();
    loop {
        if peripherals.TIM1.sr.read().uif().bit_is_set() {
            peripherals.TIM1.sr.write(|w| w.uif().clear_bit());
            peripherals.GPIOF.odr.modify(|r, w| w.odr0().bit(!r.odr0().bit()));
        }
        // delay_ms(1000);
    }
}
