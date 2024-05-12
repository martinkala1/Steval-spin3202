use stm32f0::stm32f0x1::{GPIOB, TIM1};

pub enum PwmChannel {
    Channel1,
    Channel2,
    Channel3,
}

pub struct Pwm {
    pub tim: TIM1,
    pub gpio: GPIOB,
}

impl Pwm {
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

    pub fn stop(&self) {
        // self.tim.cr1.modify(|_,w| w.cen().clear_bit());
        self.tim.ccer.modify(|_, w| w.cc1e().clear_bit());
        self.tim.ccer.modify(|_, w| w.cc2e().clear_bit());
        self.tim.ccer.modify(|_, w| w.cc3e().clear_bit());
        self.gpio.odr.modify(|_, w| w.odr13().clear_bit());
        self.gpio.odr.modify(|_, w| w.odr14().clear_bit());
        self.gpio.odr.modify(|_, w| w.odr15().clear_bit());
    }

    pub fn channel_up(&self, channel_engage: &PwmChannel, channel_disengage: &PwmChannel) {
        self.tim.cr1.modify(|_,w| w.cen().clear_bit());
        match channel_disengage {
            PwmChannel::Channel1 => self.tim.ccer.write(|w| w.cc1e().clear_bit()),
            PwmChannel::Channel2 => self.tim.ccer.write(|w| w.cc2e().clear_bit()),
            PwmChannel::Channel3 => self.tim.ccer.write(|w| w.cc3e().clear_bit()),
        };
        match channel_engage {
            PwmChannel::Channel1 => self.tim.ccer.write(|w| w.cc1e().set_bit()),
            PwmChannel::Channel2 => self.tim.ccer.write(|w| w.cc2e().set_bit()),
            PwmChannel::Channel3 => self.tim.ccer.write(|w| w.cc3e().set_bit()),
        };
        self.tim.cr1.modify(|_,w| w.cen().set_bit());
    }
    
    pub fn channel_down(&self, channel_engage: &PwmChannel, channel_disengage: &PwmChannel) {
        match channel_disengage {
            PwmChannel::Channel1 => self.gpio.odr.write(|w| w.odr13().clear_bit()),
            PwmChannel::Channel2 => self.gpio.odr.write(|w| w.odr14().clear_bit()),
            PwmChannel::Channel3 => self.gpio.odr.write(|w| w.odr15().clear_bit()),
        }
        match channel_engage {
            PwmChannel::Channel1 => self.gpio.odr.write(|w| w.odr13().set_bit()),
            PwmChannel::Channel2 => self.gpio.odr.write(|w| w.odr14().set_bit()),
            PwmChannel::Channel3 => self.gpio.odr.write(|w| w.odr15().set_bit()),
        }
    }

}