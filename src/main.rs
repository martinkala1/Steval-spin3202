#![no_std]
#![no_main]

use core::{cell::RefCell, panic::PanicInfo};

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use six_step::Motor;
use stm32f0::stm32f0x1::{self, interrupt, GPIOB, GPIOF, NVIC, TIM2};

use system_clocks::configure_sysclk_pll;
use gpio::{configure_gpiof, configure_gpiob};
use timers::{configure_tim1, configure_tim2, Pwm};

mod error_type;
mod gpio;
mod system_clocks;
mod timers;
mod six_step;


static GGPIOF: Mutex<RefCell<Option<GPIOF>>> = Mutex::new(RefCell::new(None));
static GTIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = stm32f0x1::Peripherals::take().unwrap();
    configure_sysclk_pll(&peripherals);
    hprintln!("Clock ready.").unwrap();
    configure_tim1(&peripherals);
    hprintln!("TIM1 ready.").unwrap();
    configure_tim2(&peripherals);
    hprintln!("TIM2 ready.").unwrap();
    configure_gpiof(&peripherals);
    hprintln!("GPIOF ready.").unwrap();
    configure_gpiob(&peripherals);
    hprintln!("GPIOB ready.").unwrap();

    let gpiof = peripherals.GPIOF;
    let tim2 = peripherals.TIM2;

    free(|cs| {
        GTIM2.borrow(cs).replace(Some(tim2));
        GGPIOF.borrow(cs).replace(Some(gpiof));
    });

    unsafe {
        NVIC::unmask(interrupt::TIM2);
    }

    let mut pwm = Pwm{tim: &peripherals.TIM1, gpio: &peripherals.GPIOB};
    let mut motor = Motor::new(&mut pwm);
    hprintln!("Initialization complete!").unwrap();


    peripherals.GPIOB.odr.write(|w| w.odr14().set_bit()) ;
    motor.start(true);
    motor.stop();
    loop {}
}

#[interrupt]
fn TIM2() {
    static mut GPIOF_WRAPPER: Option<GPIOF> = None;
    static mut TIM_WRAPPER: Option<TIM2> = None;
    let tim = TIM_WRAPPER.get_or_insert_with(|| {
        free(|cs| {
            GTIM2.borrow(cs).replace(None).unwrap()
        })
    });

    let gpiof = GPIOF_WRAPPER.get_or_insert_with(|| {
        free(|cs| {
            GGPIOF.borrow(cs).replace(None).unwrap()
        })
    });

    tim.sr.modify(|_,w| w.uif().clear_bit());
    gpiof.odr.modify(|r,w| w.odr0().bit(!r.odr0().bit()));

}
