use cortex_m_semihosting::hprintln;
use stm32f0::stm32f0x1::adc::tr;
use stm32f0::stm32f0x1::{Peripherals, TIM3};

use crate::system_clocks::delay_us;
use crate::{system_clocks::delay_ms, timers::Pwm};
use crate::six_step::step_sequence::STEP_COUNT;

use super::step_sequence::{Step, STEP_SEQUENCE};

pub struct Motor {
    pub ccr: u16,
    pub running: bool,
    pub pwm: Pwm,
    pub actual_step_index: usize,
    pub prev_step_index: usize,
}

impl Motor {

    pub fn new<'a>(pwm: Pwm) -> Motor {
        // pwm.set_ccr(0);
        Motor {
            ccr: 0,
            running: false,
            pwm,
            actual_step_index: 0,
            prev_step_index: 0,
        }
    }

    pub fn start(&mut self, forward: bool, tim: &TIM3) {
        hprintln!("Starting motor...").unwrap();
        const START_UP_TIME: u16 = 1000;
        if self.running == false {
            self.running = true;
            let mut delay = 4;
            self.engage_step();
            delay_ms(delay);
            let mut start_up_time: u16 = 0;
            while delay > 1 {
                self.next_step(forward);
                delay_ms(delay);
                delay -= 1;
            }
            tim.cr1.modify(|_, w| w.cen().enabled()); // motor up to starting speed, enable timer to start ADC conversions
            while start_up_time < START_UP_TIME {
                self.next_step(forward);
                delay_ms(delay);
                start_up_time += 1;
            }
        }
        self.stop(tim);
    }

    pub fn stop(&mut self, tim: &TIM3) {
        hprintln!("Stopping motor!").unwrap();
        if self.running != false {
            self.pwm.stop();
            tim.cr1.modify(|_, w| w.cen().disabled());
            self.actual_step_index = 0;
            self.prev_step_index = 0;
        }
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