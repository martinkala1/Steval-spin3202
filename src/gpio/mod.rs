mod gpio;
pub use gpio::{configure_gpiof, configure_gpiob};

static CONFIGURATION_SUCCESS: i8 = 0;