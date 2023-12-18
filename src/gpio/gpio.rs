use stm32f0::stm32f0x1::Peripherals;

use super::CONFIGURATION_SUCCESS;

pub fn configure_gpiof(p: &Peripherals) -> i32{
    let rcc = &p.RCC;
    let gpiof = &p.GPIOF;

    rcc.ahbenr.write(|w| w.iopfen().set_bit());
    gpiof.moder.write(|w| w.moder0().output().moder1().input().moder6().output()); // todo
    gpiof.odr.write(|w| w.odr6().set_bit());
    return CONFIGURATION_SUCCESS;
}

pub fn configure_gpiob(p: &Peripherals) -> i32 {
    let rcc = &p.RCC;
    let gpiob = &p.GPIOB;

    rcc.ahbenr.modify(|_, w| w.iopben().set_bit());
    gpiob.moder.modify(|_, w| w.moder13().output().moder14().output().moder15().output());
    return CONFIGURATION_SUCCESS;
}