use crate::timers::Pwm;
use crate::six_step::step_sequence::STEP_COUNT;

use super::step_sequence::{Step, STEP_SEQUENCE};

pub struct Motor {
    pub ccr: u16,
    pub pwm: Pwm,
    pub actual_step_index: usize,
    pub prev_step_index: usize,
}

impl Motor {

    pub fn new(pwm: Pwm) -> Motor {
        // pwm.set_ccr(0);
        Motor {
            ccr: 0,
            pwm,
            actual_step_index: 0,
            prev_step_index: 0,
        }
    }

    pub fn stop(&mut self) {
        // hprintln!("Stopping motor!").unwrap();
        self.pwm.stop();
        self.actual_step_index = 0;
        self.prev_step_index = 0;
    }

    pub fn set_speed(&mut self, ccr_val: u32) { // todo: calculate ccr value yourself from timer frequency
        self.pwm.set_ccr(ccr_val);
    }

    pub fn engage_step(&mut self) {
        let next_step: &Step = STEP_SEQUENCE[self.actual_step_index];
        let current_step: &Step = STEP_SEQUENCE[self.prev_step_index];

        self.pwm.channel_up(&next_step.channel_up, &current_step.channel_up);
        self.pwm.channel_down(&next_step.channel_down, &current_step.channel_down);
    }

    pub fn next_step(&mut self, forward: bool) {
        self.prev_step_index = self.actual_step_index;
        if forward == true {
            self.actual_step_index += 1;
            if self.actual_step_index >= STEP_COUNT { self.actual_step_index = 0;}
        } else {
            if self.actual_step_index == 0 { self.actual_step_index = STEP_COUNT-1; }
            else { self.actual_step_index -= 1; }
        }
        self.engage_step();
    }

}