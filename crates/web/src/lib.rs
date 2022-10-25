use std::io::Cursor;
use std::panic;

use rustagram::image;
use rustagram::image::io::Reader as ImageReader;
use rustagram::FilterType;
use rustagram::RustagramFilter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();
}

#[wasm_bindgen]
pub fn apply_filter(img: &[u8], filter: &str) -> Box<[u8]> {
    log::debug!("image: {} bytes, filter: {:?}", img.len(), filter);

    let img = ImageReader::new(Cursor::new(img))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let filter_type = filter.parse().unwrap_or(FilterType::Valencia);
    let out = img.to_rgba8().apply_filter(filter_type);
    let mut bytes: Vec<u8> = Vec::new();
    out.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .unwrap();
    bytes.into_boxed_slice()
}
