use std::io::Cursor;
use std::panic;
use wasm_bindgen::prelude::*;

use rustagram::image::io::Reader;
use rustagram::image::ImageOutputFormat;
use rustagram::RustagramFilter;

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();
}

#[wasm_bindgen]
pub fn apply_filter(img: &[u8], filter: &str) -> Vec<u8> {
    log::debug!("image: {} bytes, filter: {:?}", img.len(), filter);

    let img = Reader::new(Cursor::new(img))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let filter_type = filter.parse().unwrap();
    let out = img.to_rgba8().apply_filter(filter_type);
    let mut bytes: Vec<u8> = Vec::new();
    out.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
        .unwrap();

    bytes
}
