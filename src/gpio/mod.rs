mod gpio;
use gpio::configure_gpiof;
use gpio::configure_gpioa;
use gpio::configure_gpiob;
use stm32f0::stm32f0x1::Peripherals;

pub fn configure_gpio(p: &Peripherals) {
    configure_gpioa(p);
    configure_gpiob(p);
    configure_gpiof(p);
}
