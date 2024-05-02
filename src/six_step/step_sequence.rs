use crate::timers::PwmChannel;

pub static STEP_COUNT: usize = 6;
pub struct Step {
    pub channel_up: PwmChannel,
    pub channel_down: PwmChannel,
    pub channel_float: PwmChannel,
}

pub static STEP1: Step = Step {
    channel_up: PwmChannel::Channel1,
    channel_down: PwmChannel::Channel2,
    channel_float: PwmChannel::Channel3
};

pub static STEP2: Step = Step {
    channel_up: PwmChannel::Channel1,
    channel_down: PwmChannel::Channel3,
    channel_float: PwmChannel::Channel2
};

pub static STEP3: Step = Step {
    channel_up: PwmChannel::Channel2,
    channel_down: PwmChannel::Channel3,
    channel_float: PwmChannel::Channel1
};

pub static STEP4: Step = Step {
    channel_up: PwmChannel::Channel2,
    channel_down: PwmChannel::Channel1,
    channel_float: PwmChannel::Channel3
};

pub static STEP5: Step = Step {
    channel_up: PwmChannel::Channel3,
    channel_down: PwmChannel::Channel1,
    channel_float: PwmChannel::Channel2
};

pub static STEP6: Step = Step {
    channel_up: PwmChannel::Channel3,
    channel_down: PwmChannel::Channel2,
    channel_float: PwmChannel::Channel1
};

pub static STEP_SEQUENCE: [&Step; 6] = [&STEP1, &STEP2, &STEP3, &STEP4, &STEP5, &STEP6];