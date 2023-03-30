//! Display driver

use crate::{command::*, consts::*};
use display_interface::{DisplayError, WriteOnlyDataCommand};

const BUFFER_SIZE: usize = (DISPLAY_WIDTH as usize) * (DISPLAY_HEIGHT as usize) / 8;

/// ST7565S display driver\
///
/// - Provides two display modes:
///   - Internal Buffer Mode: This mode allows you to modify an internal buffer by using methods like [`set_pixel`], [`clear`], or by using the [`embedded-graphics`] crate. Once you have made your changes, you can call the [`flush`] method to write the buffer to the display.
///   - Direct Write Mode: This mode allows you to write directly to the display memory by calling the [`draw`] method.
///
/// [`embedded-graphics`]: https://docs.rs/embedded-graphics
/// [`set_pixel`]: crate::display::ST7567S#method.set_pixel
/// [`clear`]: crate::display::ST7567S#method.clear
/// [`flush`]: crate::display::ST7567S#method.flush
/// [`draw`]: crate::display::ST7567S#method.draw
///
pub struct ST7567S<DI> {
    display_interface: DI,
    buffer: [u8; BUFFER_SIZE],
}

impl<DI: WriteOnlyDataCommand> ST7567S<DI> {
    /// Create new instance of ST7565S driver\
    /// # Arguments
    /// * `display_interface` - The interface abstraction from `display_interface` crate
    pub fn new(display_interface: DI) -> Self {
        ST7567S {
            display_interface,
            buffer: [0; BUFFER_SIZE],
        }
    }

    /// Send init commands to the display and turn it on
    pub fn init(&mut self) -> Result<(), DisplayError> {
        SetBiasCommand::Bias1_9.write(&mut self.display_interface)?;
        SetSEGDirectionCommand::Normal.write(&mut self.display_interface)?;
        SetCOMDirectionCommand::Reverse.write(&mut self.display_interface)?;
        SetRegulationResistorRatioCommand::Ratio5_0.write(&mut self.display_interface)?;
        SetElectronicVolumeCommand::new(40)
            .unwrap()
            .write(&mut self.display_interface)?;
        SetPowerControlCommand::BoosterOn.write(&mut self.display_interface)?;
        SetPowerControlCommand::VoltageRegulatorOn.write(&mut self.display_interface)?;
        SetPowerControlCommand::VoltageFollowerOn.write(&mut self.display_interface)?;
        SetStartLineCommand::new(0)
            .unwrap()
            .write(&mut self.display_interface)?;

        self.clear();
        self.flush()?;

        DisplayOnCommand::On.write(&mut self.display_interface)?;

        Ok(())
    }

    /// Reset some display parameters to default values: Start Line, Column Address, Page Address and COM Direction\
    /// Usually doesn't need to be called
    pub fn reset(&mut self) -> Result<(), DisplayError> {
        ResetCommand.write(&mut self.display_interface)
    }

    /// Clear the display buffer
    pub fn clear(&mut self) {
        self.buffer = [0; BUFFER_SIZE];
    }

    /// Set pixel in the display buffer\
    /// Pixel coordinates starts from top left corner and goes to bottom right corner
    pub fn set_pixel(&mut self, x: u8, y: u8, value: bool) -> Result<(), DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::OutOfBoundsError);
        }

        let column: usize = x as usize;
        let page: usize = (y / 8) as usize;
        let page_bit = y % 8;

        let byte_idx = page * (DISPLAY_WIDTH as usize) + column;
        let byte = self.buffer[byte_idx];
        let bit_value: u8 = value.into();
        let byte = byte & !(1 << page_bit) | (bit_value << page_bit);

        self.buffer[byte_idx] = byte;

        Ok(())
    }

    /// Send buffer to the display\
    /// Buffer represents by 8 pages of 128 columns where 1 byte represents 8 vertical pixels
    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        if buffer.len() != BUFFER_SIZE {
            return Err(DisplayError::OutOfBoundsError);
        }

        Self::flush_buffer_chunks(
            &mut self.display_interface,
            buffer,
            (0, 0),
            (DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1),
        )
    }

    /// Send part of the buffer to the display\
    /// Buffer represents by 8 pages of 128 columns where 1 byte represents 8 vertical pixels
    ///
    /// # Arguments
    /// * `buffer` - the entire buffer from which the required part will be sent
    /// * `top_left` and `bottom_right` are coordinates of the top left and bottom right corners of the area to be drawn
    pub fn bounded_draw(
        &mut self,
        buffer: &[u8],
        top_left: (u8, u8),
        bottom_right: (u8, u8),
    ) -> Result<(), DisplayError> {
        Self::flush_buffer_chunks(&mut self.display_interface, buffer, top_left, bottom_right)
    }

    /// Send internal buffer to the display
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        Self::flush_buffer_chunks(
            &mut self.display_interface,
            self.buffer.as_slice(),
            (0, 0),
            (DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1),
        )
    }

    fn flush_buffer_chunks(
        display_interface: &mut DI,
        buffer: &[u8],
        top_left: (u8, u8),
        bottom_right: (u8, u8),
    ) -> Result<(), DisplayError> {
        if top_left.0 >= DISPLAY_WIDTH || top_left.1 >= DISPLAY_HEIGHT {
            return Err(DisplayError::OutOfBoundsError);
        }
        if bottom_right.0 >= DISPLAY_WIDTH || bottom_right.1 >= DISPLAY_HEIGHT {
            return Err(DisplayError::OutOfBoundsError);
        }
        if top_left.0 > bottom_right.0 || top_left.1 > bottom_right.1 {
            return Err(DisplayError::OutOfBoundsError);
        }

        let first_page: usize = (top_left.1 / 8) as usize;
        let first_column: usize = top_left.0 as usize;

        let last_page: usize = (bottom_right.1 / 8) as usize;
        let last_column: usize = bottom_right.0 as usize;

        buffer
            .chunks(DISPLAY_WIDTH as usize)
            .skip(first_page)
            .take(last_page - first_page + 1)
            .map(|page| &page[first_column..=last_column])
            .enumerate()
            .try_for_each(|(page_idx, page)| {
                SetPageAddressCommand::new(first_page as u8 + page_idx as u8)
                    .unwrap()
                    .write(display_interface)?;
                SetColumnAddressLSNibbleCommand::new(first_column as u8)
                    .unwrap()
                    .write(display_interface)?;
                SetColumnAddressMSNibbleCommand::new(first_column as u8)
                    .unwrap()
                    .write(display_interface)?;

                display_interface.send_data(display_interface::DataFormat::U8(page))
            })
    }
}
