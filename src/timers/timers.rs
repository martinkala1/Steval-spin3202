use stm32f0::stm32f0x1::Peripherals;

/// Configure TIM1 for PWM generation
pub fn configure_tim1(p: &Peripherals) {
    let tim = &p.TIM1;
    let gpioa = &p.GPIOA;
    let rcc = &p.RCC;

    rcc.apb2enr.write(|w| w.tim1en().set_bit()); // enable clock for TIM1
    rcc.ahbenr.write(|w|w.iopaen().set_bit()); // enable clock for GPIOA


    gpioa.moder.write(|w| w.moder8().alternate().moder9().alternate().moder10().alternate());
    gpioa.afrh.write(|w| w.afrh8().af2().afrh9().af2().afrh10().af2()); // Alternate function of gpioa pins where tim1 is connected

    unsafe {
        tim.arr.write(|w| w.bits(800)); 
        tim.ccr1().write(|w| w.bits(50)); // DC ch1
        tim.ccr2().write(|w| w.bits(50)); // DC ch2
        tim.ccr3().write(|w| w.bits(50)); // DC ch3
    }

    tim.ccmr1_output().write(|w| w.oc1ce().clear_bit().oc2ce().clear_bit().oc1pe().enabled().oc1m().pwm_mode1().oc2pe().enabled().oc2m().pwm_mode1());
    tim.ccmr2_output().write(|w| w.oc3ce().clear_bit().oc3pe().enabled().oc3m().pwm_mode1());

    tim.bdtr.write(|w| w.moe().set_bit()); // Enable main output of OC channels

    tim.cr1.write(|w| w.arpe().set_bit()); // Enable auto-reload preload registers
    tim.egr.write(|w| w.ug().set_bit()); // Enable event generation
    tim.cr1.write(|w| w.cen().set_bit()); // enable timer
}

pub fn configure_tim2(p: &Peripherals) {
    let tim = &p.TIM2;
    let rcc = &p.RCC;

    rcc.apb1enr.write(|w| w.tim2en().set_bit()); // enable clock for TIM2

    tim.egr.write(|w| w.ug().set_bit());
    unsafe {
        tim.psc.write(|w| w.bits(999));
    }
    tim.arr.write(|w| w.bits(16_000)); // overflow every 0.5 seconds

    tim.dier.modify(|_,w| w.uie().set_bit());
    tim.cr1.write(|w| w.cen().set_bit()); // enable timer
    tim.sr.write(|w| w.uif().clear_bit());
}