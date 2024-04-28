mod clocks;
pub use clocks::delay_ms;
use clocks::__configure_sysclk_pll;

use stm32f0::stm32f0x1::Peripherals;
use crate::error_type::{ConfigErrType, LogError};

pub fn configure_sysclk_pll(p: &Peripherals) {
    __configure_sysclk_pll(p).unwrap_or_else(|config_error| {
        match config_error {
            ConfigErrType::HsiEnableTimeout => {
                config_error.log_error();
                loop {}
            },
            _ => {
                config_error.log_error()
            }
        }
    });
}