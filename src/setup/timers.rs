use stm32f0::stm32f0x1::Peripherals;

pub fn configure_tim1(p: &Peripherals) {
    let tim = &p.TIM1;

    tim.egr.write(|w| w.ug().set_bit());
    tim.psc.write(|w| unsafe { w.bits(1000) });
    tim.arr.write(|w| unsafe { w.bits(16000) });
    tim.cr1.write(|w| w.cen().set_bit());
    tim.sr.write(|w| w.uif().clear_bit());
}