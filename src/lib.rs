//! Extract significant colors from images
//!
//! The idea the extract 'vibrant' colors is adopted from the palette library from Android (by
//! Google). The implementation is based on the excellent work of [vibrant-js] and [color_quant].
//!
//! [vibrant-js]: https://github.com/jariz/vibrant.js
//! [color_quant]: https://github.com/PistonDevelopers/color_quant



#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]


#![deny(missing_docs)]

extern crate image;

mod settings;
mod palette;
mod vibrant;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
