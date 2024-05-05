use stm32f0::stm32f0x1::Peripherals;

pub fn configure_usart(p: &Peripherals) {
    let rcc = &p.RCC;
    let usart = &p.USART1;

    rcc.apb2enr.modify(|_, w| w.usart1en().enabled());

    usart.cr1.write(|w| w.m0().bit8()); // 8 bit word
    usart.cr2.write(|w| w.stop().stop1()); // 1 stop bit
    usart.brr.write(|w| w.brr().bits(9600)); // 9600 baudrate

    usart.cr1.modify(|_, w| w.ue().enabled().re().enabled()); // enable usart, enable receiver
}