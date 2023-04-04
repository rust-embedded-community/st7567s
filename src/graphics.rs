//! [`embedded-graphics`](https://docs.rs/embedded-graphics) support

use crate::{
    consts::*,
    display::{BufferedMode, ST7567S},
};
use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::{Dimensions, OriginDimensions, Size},
    Pixel,
};

impl<DI: WriteOnlyDataCommand> OriginDimensions for ST7567S<DI, BufferedMode> {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH.into(), DISPLAY_HEIGHT.into())
    }
}

impl<DI: WriteOnlyDataCommand> DrawTarget for ST7567S<DI, BufferedMode> {
    type Color = BinaryColor;

    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bounding_box = self.bounding_box();
        for Pixel(point, color) in pixels.into_iter() {
            if bounding_box.contains(point) {
                self.set_pixel(point.x as u8, point.y as u8, color.is_on())?;
            }
        }

        Ok(())
    }
}
