use stm32f0::stm32f0x1::Peripherals;

pub fn configure_gpiof(p: &Peripherals) {
    let rcc = &p.RCC;
    let gpiof = &p.GPIOF;

    rcc.ahbenr.modify(|_, w| w.iopfen().set_bit());
    gpiof.moder.write(|w| w.moder0().output().moder1().input().moder6().output()); // todo: overcurrent protection
    gpiof.odr.write(|w| w.odr6().set_bit()); // PF6 set to one to increase overcurrent threshold
    gpiof.odr.modify(|_, w| w.odr0().set_bit()); // turn LED off
}

pub fn configure_gpiob(p: &Peripherals) {
    let rcc = &p.RCC;
    let gpiob = &p.GPIOB;

    rcc.ahbenr.modify(|_, w| w.iopben().set_bit());
    gpiob.moder.modify(|_, w| w.moder13().output().moder14().output().moder15().output());
}

pub fn configure_gpioa(p: &Peripherals) {
    let gpioa = &p.GPIOA;
    let rcc = &p.RCC;

    rcc.ahbenr.modify(|_,w| w.iopaen().set_bit()); // enable clock for GPIOA

    gpioa.moder.write(|w| w.moder8().alternate().moder9().alternate().moder10().alternate().moder0().analog().moder1().analog().moder2().analog());
    gpioa.afrh.write(|w| w.afrh8().af2().afrh9().af2().afrh10().af2()); // Alternate function of gpioa pins where tim1 is connected
}