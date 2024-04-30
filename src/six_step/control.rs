use cortex_m_semihosting::hprintln;

use crate::{system_clocks::delay_ms, timers::Pwm};
use crate::six_step::step_sequence::STEP_COUNT;

use super::step_sequence::{Step, STEP_SEQUENCE};

pub struct Motor<'a> {
    pub ccr: u16,
    pub running: i8,
    pwm: &'a mut Pwm<'a>,
    actual_step_index: usize,
    prev_step_index: usize,
}

impl Motor<'_> {

    pub fn new<'a>(pwm: &'a mut Pwm<'a>) -> Motor<'a> {
        // pwm.set_ccr(0);
        Motor {
            ccr: 0,
            running: 0,
            pwm,
            actual_step_index: 0,
            prev_step_index: 0,
        }
    }

    pub fn start(&mut self, forward: bool) {
        hprintln!("Starting motor...").unwrap();
        const START_UP_TIME: u16 = 5000;
        if self.running == 0 {
            self.running = 1;
            let mut delay = 10;
            self.engage_step();
            delay_ms(delay);
            let mut start_up_time: u16 = 0;
            while delay > 0 {
                self.next_step(forward);
                delay_ms(delay);
                delay -= 1;
            }
            while start_up_time < START_UP_TIME {
                self.next_step(forward);
                start_up_time += 1;
            }

            // TODO - Enable interrupts in between these steps
            self.next_step(forward);
            delay_ms(delay);
            self.next_step(forward);
            delay_ms(delay);
            self.next_step(forward);
            delay_ms(delay);
            self.next_step(forward);
            delay_ms(delay);
            self.next_step(forward);
            delay_ms(delay);
            self.next_step(forward);
        }
    }

    pub fn stop(&mut self) {
        hprintln!("Stopping motor!").unwrap();
        if self.running != 0 {
            self.pwm.stop();
            self.actual_step_index = 0;
            self.prev_step_index = 0;
        }
    }

    pub fn set_speed(&mut self, ccr_val: u32) { // todo: calculate ccr value yourself from timer frequency
        self.pwm.set_ccr(ccr_val);
    }

    fn engage_step(&mut self) {
        let next_step: &Step = STEP_SEQUENCE[self.actual_step_index];
        let current_step: &Step = STEP_SEQUENCE[self.prev_step_index];

        self.pwm.channel_up(&next_step.channel_up, &current_step.channel_up);
        self.pwm.channel_down(&next_step.channel_down, &current_step.channel_down);
    }

    fn next_step(&mut self, forward: bool) {
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