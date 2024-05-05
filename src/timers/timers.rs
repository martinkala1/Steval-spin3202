use stm32f0::stm32f0x1::Peripherals;

/// Configure TIM1 for PWM generation
pub fn configure_tim1(p: &Peripherals) {
    let tim = &p.TIM1;
    let rcc = &p.RCC;

    rcc.apb2enr.write(|w| w.tim1en().set_bit()); // enable clock for TIM1
    rcc.ahbenr.write(|w|w.iopaen().set_bit()); // enable clock for GPIOA


    unsafe {
        tim.arr.write(|w| w.bits(800)); 
        tim.ccr1().write(|w| w.bits(50)); // DC ch1
        tim.ccr2().write(|w| w.bits(50)); // DC ch2
        tim.ccr3().write(|w| w.bits(50)); // DC ch3
    }

    tim.cr2.modify(|_,w| w.mms().update());
    tim.ccmr1_output().write(|w| w.oc1ce().clear_bit().oc2ce().clear_bit().oc1pe().enabled().oc1m().pwm_mode1().oc2pe().enabled().oc2m().pwm_mode1());
    tim.ccmr2_output().write(|w| w.oc3ce().clear_bit().oc3pe().enabled().oc3m().pwm_mode1());

    tim.bdtr.write(|w| w.moe().set_bit()); // Enable main output of OC channels

    tim.cr1.write(|w| w.arpe().set_bit()); // Enable auto-reload preload registers
    tim.cr1.write(|w| w.cen().set_bit()); // enable timer
}

// pub fn configure_tim2(p: &Peripherals) {
//     let tim = &p.TIM2;
//     let rcc = &p.RCC;

//     rcc.apb1enr.modify(|_, w| w.tim2en().set_bit()); // enable clock for TIM2
//     tim.cr1.modify(|_, w| w.opm().enabled());
//     tim.dier.write(|w| w.uie().enabled());

//     unsafe {
//         tim.psc.write(|w| w.bits(800)); // every tick is 50us
//     }
//     tim.arr.write(|w| w.bits(0)); // we do not want overflow

//     // tim.cr1.write(|w| w.cen().set_bit()); // enable timer
// }