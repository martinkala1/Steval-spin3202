use stm32f0::stm32f0x1::TIM1;


pub enum PwmChannel {
    Channel1,
    Channel2,
    Channel3,
}

pub struct Pwm<'a> {
    pub tim: &'a TIM1,
}

impl Pwm<'_> {
    pub fn get_duty_cycle(&self, channel: PwmChannel) -> f32 {
        let arr = self.tim.arr.read().bits();
        match channel {
            PwmChannel::Channel1 => return self.tim.ccr1().read().bits() as f32/arr as f32,
            PwmChannel::Channel2 => return self.tim.ccr2().read().bits() as f32/arr as f32,
            PwmChannel::Channel3 => return self.tim.ccr3().read().bits() as f32/arr as f32,
        };
    }

    pub fn set_ccr(&mut self, mut ccr_val: u32) {
        let arr_val = self.tim.arr.read().bits() as u32;
        if ccr_val > arr_val {
            ccr_val = arr_val;
        }
        unsafe {
            self.tim.ccr1().write(|w| w.bits(ccr_val));
            self.tim.ccr2().write(|w| w.bits(ccr_val));
            self.tim.ccr3().write(|w| w.bits(ccr_val));
        }
    }

    pub fn is_enabled(&self, channel: PwmChannel) -> bool {
        match channel {
            PwmChannel::Channel1 => return self.tim.ccer.read().cc1e().bit_is_set(),
            PwmChannel::Channel2 => return self.tim.ccer.read().cc2e().bit_is_set(),
            PwmChannel::Channel3 => return self.tim.ccer.read().cc3e().bit_is_set(),
        };
    }

    pub fn pwm_start(&self, channel: PwmChannel) {
        self.tim.cr1.modify(|_,w| w.cen().clear_bit());
        match channel {
            PwmChannel::Channel1 => self.tim.ccer.write(|w| w.cc1e().set_bit()),
            PwmChannel::Channel2 => self.tim.ccer.write(|w| w.cc2e().set_bit()),
            PwmChannel::Channel3 => self.tim.ccer.write(|w| w.cc3e().set_bit()),
        };
        self.tim.cr1.modify(|_,w| w.cen().set_bit());
    }

    pub fn pwm_stop(&self, channel: PwmChannel) {
        self.tim.cr1.modify(|_,w| w.cen().clear_bit());
        match channel {
            PwmChannel::Channel1 => self.tim.ccer.write(|w| w.cc1e().clear_bit()),
            PwmChannel::Channel2 => self.tim.ccer.write(|w| w.cc2e().clear_bit()),
            PwmChannel::Channel3 => self.tim.ccer.write(|w| w.cc3e().clear_bit()),
        };
        self.tim.cr1.modify(|_,w| w.cen().set_bit());
    }
}