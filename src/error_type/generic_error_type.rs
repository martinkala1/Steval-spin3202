
use cortex_m_semihosting::hprintln;

pub trait LogError {
    fn log_error(&self);
}

// Has to be implemented as enum, semihosting console can only log literals
pub enum GenericError {
}

impl LogError for GenericError {
    fn log_error(&self) {
        // TODO
    }
}

pub enum ConfigurationErrorType {
        HsiEnableTimeout,
        PllReadyTimeout,
        PllEnableTimeout,
}

impl LogError for ConfigurationErrorType {
    fn log_error(&self) {
        match self {
            ConfigurationErrorType::HsiEnableTimeout => hprintln!("Enabling HSI timed out, exiting application.").unwrap(),
            ConfigurationErrorType::PllReadyTimeout => hprintln!("PLL timed out, falling back to 8 MHz SysClk frequency.").unwrap(),
            ConfigurationErrorType::PllEnableTimeout => hprintln!("PLL enabling timed out, falling back to 8 MHz SysClk frequency.").unwrap(),
        }
    }
}
