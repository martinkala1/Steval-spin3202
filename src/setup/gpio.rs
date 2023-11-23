use stm32f0::stm32f0x1::Peripherals;

use super::CONFIGURATION_SUCCESS;

pub fn configure_gpiof(p: &Peripherals) -> i32{
    let rcc = &p.RCC;
    let gpiof = &p.GPIOF;

    rcc.ahbenr.write(|w| w.iopfen().set_bit());
    gpiof.moder.write(|w| w.moder0().output().moder1().input());
    return CONFIGURATION_SUCCESS;
}