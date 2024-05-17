use stm32f0::stm32f0x1::{Peripherals, USART1};

use crate::system_clocks::CPU_FREQ;

pub fn uart_send(uart: &USART1, msg: &str) {
    while uart.isr.read().txe().bit_is_clear() {}
    for chr in msg.as_bytes() {
        unsafe {
            uart.tdr.write(|w| w.bits(*chr as u32));
        }
        while uart.isr.read().tc().bit_is_clear() {}
    }
}

fn __uart_read(uart: &USART1) -> char {
    uart.rdr.read().bits() as u8 as char
}

pub fn uart_read_async(uart: &USART1) -> char {
    if uart.isr.read().rxne().bit_is_set() {
        return __uart_read(uart);
    }
    return '\0';
}

pub fn uart_read(uart: &USART1) -> char {
    while uart.isr.read().rxne().bit_is_clear() {}
    __uart_read(uart)
}
pub fn configure_uart(p: &Peripherals) {
    let rcc = &p.RCC;
    let usart = &p.USART1;
    let baud_rate: u32 = 38_400;
    rcc.apb2enr.modify(|_, w| w.usart1en().enabled());

    usart.cr1.write(|w| w.m0().bit8()); // 8 bit word
    usart.cr2.write(|w| w.stop().stop1()); // 1 stop bit
    usart.brr.write(|w| w.brr().bits((CPU_FREQ / baud_rate) as u16));

    usart.cr1.modify(|_, w| w.ue().enabled().re().enabled().te().enabled()); // enable usart, enable receiver
}