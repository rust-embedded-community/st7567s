# ST7567S Display Controller Driver

[![Crates.io](https://img.shields.io/crates/v/st7567s.svg)](https://crates.io/crates/st7567s)
[![Docs.rs](https://docs.rs/st7567s/badge.svg)](https://docs.rs/st7567s)

This crate provides a driver for the ST7567S display controller that can be used with Rust embedded projects.

# Features

- Supports I2C and SPI communication protocols via the [`display_interface`](https://docs.rs/display_interface) crate. 
- Provides two display modes:
  - Internal Buffer Mode: This mode allows you to modify an internal buffer by using methods like `set_pixel`, `clear`, or by using the [`embedded-graphics`](https://docs.rs/embedded-graphics) crate. Once you have made your changes, you can call the `flush` method to write the buffer to the display.
  - Direct Write Mode: This mode allows you to write directly to the display memory by calling the `draw` method.

**Note**: This driver is designed to work with a more generic 128x64 resolution, instead of the original 132x65 resolution of the ST7567S controller.  
**Note**: SPI communication is not tested yet.

Thanks [`ssd1306`](https://github.com/jamwaffles/ssd1306) driver for served as an example.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

