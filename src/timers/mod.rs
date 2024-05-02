mod timers;
pub use timers::{configure_tim1, configure_tim2};

mod pwm;
pub use pwm::{Pwm, PwmChannel};