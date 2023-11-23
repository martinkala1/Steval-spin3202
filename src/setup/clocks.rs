
use cortex_m::asm::delay;
use stm32f0::stm32f0x1::Peripherals;

use crate::error_type::{ConfigurationErrorType, LogError};

static mut CPU_FREQ: u32 = 8_000_000;

const CLK_CONFIG_TIMEOUT: u32 = 4_000_000;

const COMPENSATION: u32 = 2; // IMHO, delay() fn takes 2 ICs per cycle waited
pub fn delay_ms(value: u32) {
    delay(value * (unsafe { CPU_FREQ / COMPENSATION } / 1000));
}

pub fn __configure_sysclk_pll(p: &Peripherals) -> Result<(), ConfigurationErrorType> {
    let rcc = &p.RCC;
    let mut cnt = 0;
    let err: ConfigurationErrorType;
    if rcc.cfgr.read().sws().is_pll() { // if pll is on
        enable_hsi(p)?;
    }
    rcc.cr.write(|w| w.pllon().clear_bit()); // disable pll
    while rcc.cr.read().pllrdy().is_ready() { // wait until pllrdy is cleared
        cnt += 1;
        if cnt > CLK_CONFIG_TIMEOUT {
            err = ConfigurationErrorType::PllReadyTimeout;
            match enable_hsi(p) {
                Ok(_) => { return Err(err);},
                Err(_) => { 
                    err.log_error();
                    return Err(ConfigurationErrorType::HsiEnableTimeout);
                },
            };
        }
    }
    cnt = 0;
    rcc.cfgr.write(|w| w.pllmul().bits(0b0010).pllsrc().hsi_div2()); // set pllmul to 4x
    rcc.cr.write(|w| w.pllon().set_bit());
    while rcc.cr.read().pllrdy().is_not_ready() {
        cnt += 1;
        if cnt > CLK_CONFIG_TIMEOUT {
            err = ConfigurationErrorType::PllReadyTimeout;
            match enable_hsi(p) {
                Ok(_) => { return Err(err);},
                Err(_) => { 
                    err.log_error();
                    return Err(ConfigurationErrorType::HsiEnableTimeout);
                },
            };
        }
    }
    cnt = 0;
    rcc.cfgr.write(|w| w.sw().pll()); // switch to pll
    while !rcc.cfgr.read().sws().is_pll() { // wait until switch is complete
        cnt += 1;
        if cnt > CLK_CONFIG_TIMEOUT {
            err = ConfigurationErrorType::PllEnableTimeout;
            match enable_hsi(p) {
                Ok(_) => { return Err(err);},
                Err(_) => { 
                    err.log_error();
                    return Err(ConfigurationErrorType::HsiEnableTimeout);
                },
            };
        }
    }
    unsafe { CPU_FREQ = (4*4)*1_000_000 };
    return Ok(());
}

fn enable_hsi(p: &Peripherals) -> Result<(), ConfigurationErrorType> {
    p.RCC.cfgr.write(|w| w.sw().hsi());
    let mut cnt: u32 = 0;
    while !p.RCC.cfgr.read().sws().is_hsi() {
        cnt += 1;
        if cnt > CLK_CONFIG_TIMEOUT {
            return Err(ConfigurationErrorType::HsiEnableTimeout);
        }
    }
    return Ok(());
}