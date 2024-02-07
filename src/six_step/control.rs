
use crate::timers::Pwm;

pub struct Motor<'a> {
    pub ccr: u32,
    pub running: i32,
    pwm: &'a mut Pwm<'a>,
}

impl Motor<'_> {

    pub fn new<'a>(pwm: &'a mut Pwm<'a>) -> Motor<'a> {
        Motor {
            ccr: 0,
            running: 0,
            pwm
        }
    }

    pub fn start(&mut self) {
        if self.running == 0 {
            // TODO
        }
    }

    pub fn stop(&mut self) {
        if self.running != 0 {
            // TODO
        }
    }

    pub fn set_speed(&mut self, ccr_val: u32) {
        self.pwm.set_ccr(ccr_val);
    }
}