//! [`embedded-graphics`](https://docs.rs/embedded-graphics) support

use crate::{consts::*, display::ST7567S};
use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::{OriginDimensions, Size},
    Pixel,
};

impl<DI: WriteOnlyDataCommand> OriginDimensions for ST7567S<DI> {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH.into(), DISPLAY_HEIGHT.into())
    }
}

impl<DI: WriteOnlyDataCommand> DrawTarget for ST7567S<DI> {
    type Color = BinaryColor;

    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if point.x < 0 || point.y < 0 {
                continue;
            }
            if point.x >= DISPLAY_WIDTH as i32 || point.y >= DISPLAY_HEIGHT as i32 {
                continue;
            }

            self.set_pixel(point.x as u8, point.y as u8, color.is_on())?;
        }

        Ok(())
    }
}
