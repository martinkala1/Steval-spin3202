
use stm32f0::stm32f0x1::Peripherals;

/// ADC runs with dedicated 14MHz HSI clock
pub fn configure_adc(p: &Peripherals) {
    let adc = &p.ADC;
    let rcc = &p.RCC;

    rcc.apb2enr.modify(|_, w| w.adcen().set_bit()); // enable ADC clock
    adc.cfgr2.write(|w| w.ckmode().adcclk());

    adc.isr.write(|w| w.adrdy().clear());
    adc.cr.modify(|_,w| w.aden().set_bit()); // enable ADC
    while !adc.isr.read().adrdy().is_ready() {}; // wait until ADC ready

    adc.chselr.write(|w| w.chsel0().selected().chsel1().selected().chsel2().selected()); // select channels to convert
    adc.cfgr1.write(|w| w.cont().single().exten().rising_edge().extsel().tim3_trgo()); // setup conversion start on tim3 update event
    adc.smpr.write(|w| w.smp().cycles239_5()); // TODO: select sampling time 
    adc.ier.write(|w| w.eocie().enabled()); // enable end of conversion sequence interrupt    

    adc.cr.modify(|_,w| w.adstart().start_conversion()); // start conversion
    adc.isr.write(|w| w.eoc().clear().eoseq().clear());
}
