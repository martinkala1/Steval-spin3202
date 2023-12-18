mod timers;
pub use timers::configure_tim1;

mod pwm;
pub use pwm::{Pwm, PwmChannel};