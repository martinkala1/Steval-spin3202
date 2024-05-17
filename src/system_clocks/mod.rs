mod clocks;
pub use clocks::{delay_ms, CPU_FREQ};
use clocks::__configure_sysclk_pll;

use stm32f0::stm32f0x1::Peripherals;

pub fn configure_sysclk_pll(p: &Peripherals) {
    __configure_sysclk_pll(p).unwrap_or_else(|_| {
        panic!("Clock configuration failure.");
    });
}