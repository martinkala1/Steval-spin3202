use core::cell::RefCell;

use cortex_m::{asm::delay, interrupt::{free, Mutex}};
use stm32f0::stm32f0x1::Peripherals;

use crate::error_type::{ConfigErrType, LogError};

static CPU_FREQ: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(8_000_000));

const CLK_CONFIG_TIMEOUT: u32 = 4_000;

const COMPENSATION: u32 = 2; // IMHO, delay() fn takes 2 ICs per cycle waited

pub fn delay_ms(value: u32) {
    let mut cpu_f = 0;
    free(|cs| cpu_f = *CPU_FREQ.borrow(cs).borrow());
    delay(value * (( cpu_f/ COMPENSATION) / 1000));
}

/// Set sysclk to PLL, resulting frequency set at 16MHz
pub fn __configure_sysclk_pll(p: &Peripherals) -> Result<(), ConfigErrType> {
    let rcc = &p.RCC;
    if rcc.cfgr.read().sws().is_pll() { // if pll is on
        enable_hsi(p);
    }
    rcc.cr.write(|w| w.pllon().clear_bit()); // disable pll
    wait_until(|| rcc.cr.read().pllrdy().is_not_ready(), ConfigErrType::PllReadyTimeout, p)?;
    rcc.cfgr.write(|w| w.pllmul().bits(0b0010).pllsrc().hsi_div2()); // set pllmul to 4x
    rcc.cr.write(|w| w.pllon().set_bit());
    wait_until(|| !rcc.cr.read().pllrdy().is_not_ready(), ConfigErrType::PllReadyTimeout, p)?;
    rcc.cfgr.write(|w| w.sw().pll()); // switch to pll
    wait_until(|| rcc.cfgr.read().sws().is_pll(), ConfigErrType::PllEnableTimeout, p)?;
    free(|cs| CPU_FREQ.borrow(cs).replace(16_000_000));
    return Ok(());
}

fn enable_hsi(p: &Peripherals) {
    p.RCC.cfgr.write(|w| w.sw().hsi());
    let mut cnt: u32 = 0;
    while !p.RCC.cfgr.read().sws().is_hsi() {
        cnt += 1;
        if cnt > CLK_CONFIG_TIMEOUT {
            ConfigErrType::HsiEnableTimeout.log_error();
            panic!("Clock configuration failure!");
        }
    }
}

fn wait_until<G>(f: G, err: ConfigErrType, p: &Peripherals) -> Result<(), ConfigErrType> where
    G: Fn() -> bool
{
    let mut cnt = 0;
    while !f() {
        cnt += 1;
        if cnt > CLK_CONFIG_TIMEOUT {
            enable_hsi(p);
            return Err(err);
        }
    }
    return Ok(());
}