use stm32f0::stm32f0x1::Peripherals;

pub fn configure_usart(p: &Peripherals) {
    let rcc = &p.RCC;
    let usart = &p.USART1;

    rcc.apb2enr.modify(|_, w| w.usart1en().enabled());

    
}