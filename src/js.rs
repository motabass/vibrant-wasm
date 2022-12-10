use std::convert::TryInto;
use std::fmt;

use image::{Pixel, Rgb};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::vibrant::Vibrancy;
use crate::utils::set_panic_hook;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Serialize, Deserialize)]
pub struct ColorPalette {
    primary: [u8; 3],
    dark: [u8; 3],
    light: [u8; 3],
    muted: [u8; 3],
    dark_muted: [u8; 3],
    light_muted: [u8; 3],
}

#[wasm_bindgen]
pub fn get_palette_from_byte_array(image_bytes: &[u8]) -> JsValue {

    let now = instant::Instant::now();

    set_panic_hook();

    // log!("Got bytes from JS");

    let image = image::load_from_memory(image_bytes)
        .ok()
        .expect("Could not load image!");

    // log!("Made image from bytes");

    let vibrancy = Vibrancy::new(&image);

    // log!("Generated colors");
    // log!("{}", vibrancy);

    let palette = ColorPalette {
        primary: vibrancy
            .primary
            .unwrap_or(Rgb([0, 0, 0]))
            .channels()
            .try_into()
            .expect("Zu viele Farben für RGB"),
        dark: vibrancy
            .dark
            .unwrap_or(Rgb([0, 0, 0]))
            .channels()
            .try_into()
            .expect("Zu viele Farben für RGB"),
        light: vibrancy
            .light
            .unwrap_or(Rgb([0, 0, 0]))
            .channels()
            .try_into()
            .expect("Zu viele Farben für RGB"),
        muted: vibrancy
            .muted
            .unwrap_or(Rgb([0, 0, 0]))
            .channels()
            .try_into()
            .expect("Zu viele Farben für RGB"),
        dark_muted: vibrancy
            .dark_muted
            .unwrap_or(Rgb([0, 0, 0]))
            .channels()
            .try_into()
            .expect("Zu viele Farben für RGB"),
        light_muted: vibrancy
            .light_muted
            .unwrap_or(Rgb([0, 0, 0]))
            .channels()
            .try_into()
            .expect("Zu viele Farben für RGB"),
    };
    let elapsed = now.elapsed();
    log!("Elapsed: {:.2?}", elapsed);
    serde_wasm_bindgen::to_value(&palette).expect("Konnte wert nicht zu JS senden")
}
