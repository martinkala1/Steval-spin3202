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

    tim.arr.write(|w| unsafe { w.bits(400) }); 

    tim.ccr1().write(|w| unsafe { w.bits(200) }); // DC 50% ch1
    tim.ccr2().write(|w| unsafe { w.bits(200) }); // DC 50% ch2
    tim.ccr3().write(|w| unsafe { w.bits(200) }); // DD 50% ch3

    tim.ccmr1_output().write(|w| w.oc1m().pwm_mode1().oc1pe().set_bit()); // Configure channel 1 as output pwm mode1, enable preload on channel 1
    tim.ccmr1_output().write(|w| w.oc2m().pwm_mode1().oc2pe().set_bit()); // Configure channel 2 as output pwm mode1, enable preload on channel 2
    tim.ccmr2_output().write(|w| w.oc3m().pwm_mode1().oc3pe().set_bit()); // Configure channel 3 as output pwm mode1, enable preload on channel 3

    tim.bdtr.write(|w| w.moe().set_bit()); // Enable main output of OC channels
    // tim.ccer.write(|w| w.cc1e().set_bit()); // enable channel 1

    tim.cr1.write(|w| w.arpe().set_bit()); // Enable auto-reload preload registers (maybe not needed)
    tim.egr.write(|w| w.ug().set_bit()); // Enable event generation

    tim.cr1.write(|w| w.cen().set_bit().cms().center_aligned1()); // enable timer and center aligned mode 
}
