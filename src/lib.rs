//! # ST7567S Display Controller Driver
//!
//! This crate provides a driver for the ST7567S display controller that can be used with Rust embedded projects.
//!
//! # Features
//!
//! - Supports I2C and SPI communication protocols via the [`display_interface`](https://docs.rs/display_interface) crate.
//! - Provides two display modes:
//!   - Internal Buffer Mode: This mode allows you to modify an internal buffer by using methods like [`set_pixel`], [`clear`], or by using the [`embedded-graphics`] crate. Once you have made your changes, you can call the [`flush`] method to write the buffer to the display.
//!   - Direct Write Mode: This mode allows you to write directly to the display memory by calling the [`draw`] method.
//!
//! [`embedded-graphics`]: https://docs.rs/embedded-graphics
//! [`set_pixel`]: crate::display::ST7567S#method.set_pixel
//! [`clear`]: crate::display::ST7567S#method.clear
//! [`flush`]: crate::display::ST7567S#method.flush
//! [`draw`]: crate::display::ST7567S#method.draw
//!
//! **Note**: This driver is designed to work with a more generic 128x64 resolution, instead of the original 132x65 resolution of the ST7567S controller.
//! **Note**: SPI communication is not tested yet.

#![no_std]

mod command;
mod consts;
pub mod display;
#[cfg(feature = "graphics")]
pub mod graphics;
pub mod interface;
