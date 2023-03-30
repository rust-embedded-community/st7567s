//! Display interface convienience factory methods

pub use display_interface_i2c::I2CInterface;

/// Wrapper for creating an I2CInterface with device-specific parameters
pub struct I2CDisplayInterface;

impl I2CDisplayInterface {
    /// Create a new I2CInterface for the ST7567S display
    pub fn new<I2C>(i2c: I2C) -> I2CInterface<I2C>
    where
        I2C: embedded_hal::blocking::i2c::Write,
    {
        I2CInterface::new(i2c, 0x3f, 0x40)
    }
}
