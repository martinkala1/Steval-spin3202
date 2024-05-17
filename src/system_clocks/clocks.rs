use cortex_m::asm::delay;
use stm32f0::stm32f0x1::Peripherals;

pub static CPU_FREQ: u32 = 16_000_000;

const CLK_CONFIG_TIMEOUT: u32 = 4_000;

const CPU_CYCLES_PER_MS: u32 = 8000; // IMHO, delay() fn takes 2 ICs per cycle waited
const CPU_CYCLES_PER_US: u32 = 8;

pub fn delay_ms(value: u32) {
    delay(value * CPU_CYCLES_PER_MS);
}

/// Set sysclk to PLL, resulting frequency set at 16MHz
pub fn __configure_sysclk_pll(p: &Peripherals) -> Result<(), ()> {
    let rcc = &p.RCC;
    rcc.cr.write(|w| w.pllon().clear_bit()); // disable pll
    wait_until(|| rcc.cr.read().pllrdy().is_not_ready())?;
    rcc.cfgr.write(|w| w.pllmul().bits(0b0010).pllsrc().hsi_div2()); // set pllmul to 4x
    rcc.cr.write(|w| w.pllon().set_bit());
    wait_until(|| !rcc.cr.read().pllrdy().is_not_ready())?;
    rcc.cfgr.write(|w| w.sw().pll()); // switch to pll
    wait_until(|| rcc.cfgr.read().sws().is_pll())?;
    return Ok(());
}

fn wait_until<G>(f: G) -> Result<(), ()> where
    G: Fn() -> bool
{
    let mut cnt = 0;
    while !f() {
        cnt += 1;
        if cnt > CLK_CONFIG_TIMEOUT {
            return Err(());
        }
    }
    return Ok(());
}