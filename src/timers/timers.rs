use stm32f0::stm32f0x1::Peripherals;

/// Configure TIM1 for PWM generation
/// 
/// Center-aligned mode, 20 KHz frequency (adjusted for center-aligned mode)
/// 
/// Default DC at 50%
/// 
/// ARR = 400
pub fn configure_tim1(p: &Peripherals) {
    let tim = &p.TIM1;
    let gpioa = &p.GPIOA;
    let rcc = &p.RCC;

    rcc.apb2enr.write(|w| w.tim1en().set_bit()); // enable clock for TIM1
    rcc.ahbenr.write(|w|w.iopaen().set_bit());


    gpioa.moder.write(|w| w.moder8().alternate().moder9().alternate().moder10().alternate());
    gpioa.afrh.write(|w| w.afrh8().af2().afrh9().af2().afrh10().af2()); // Alternate function of gpioa pins where tim1 is connected

    unsafe {
        tim.arr.write(|w| w.bits(400)); 
        tim.ccr1().write(|w| w.bits(360)); // DC 50% ch1
        tim.ccr2().write(|w| w.bits(360)); // DC 50% ch2
        tim.ccr3().write(|w| w.bits(360)); // DC 50% ch3
    }

    tim.ccmr1_output().write(|w| w.oc1ce().clear_bit().oc2ce().clear_bit().oc1pe().enabled().oc1m().pwm_mode1().oc2pe().enabled().oc2m().pwm_mode1());
    tim.ccmr2_output().write(|w| w.oc3ce().clear_bit().oc3pe().enabled().oc3m().pwm_mode1());
    // tim.ccmr1_output().modify(|_, w| w.oc1m().pwm_mode1().oc1pe().enabled()); // Configure channel 1 as output pwm mode1, enable preload on channel 1
    // tim.ccmr1_output().modify(|_, w| w.oc2m().pwm_mode1().oc2pe().enabled()); // Configure channel 2 as output pwm mode1, enable preload on channel 2
    // tim.ccmr2_output().modify(|_, w| w.oc3m().pwm_mode1().oc3pe().enabled()); // Configure channel 3 as output pwm mode1, enable preload on channel 3

    tim.bdtr.write(|w| w.moe().set_bit()); // Enable main output of OC channels
    // tim.ccer.write(|w| w.cc1e().set_bit()); // enable channel 1

    tim.cr1.write(|w| w.arpe().set_bit()); // Enable auto-reload preload registers
    tim.egr.write(|w| w.ug().set_bit()); // Enable event generation
    tim.cr1.write(|w| w.cen().set_bit()); // enable timer
}
