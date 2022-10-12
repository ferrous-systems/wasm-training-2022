use std::io::Cursor;
use std::panic;

use rustagram::image;
use rustagram::image::io::Reader as ImageReader;
use rustagram::FilterType::{self, *};
use rustagram::RustagramFilter;

use wasm_bindgen::prelude::*;

const FILTER_STRINGS: &[&str] = &[
    "1977",
    "aden",
    "brannan",
    "brooklyn",
    "clarendon",
    "earlybird",
    "gingham",
    "hudson",
    "inkwell",
    "kelvin",
    "lark",
    "lofi",
    "maven",
    "mayfair",
    "moon",
    "nashville",
    "reyes",
    "rise",
    "slumber",
    "stinson",
    "toaster",
    "valencia",
    "walden",
];
const FILTER_TYPES: &[FilterType] = &[
    NineTeenSeventySeven,
    Aden,
    Brannan,
    Brooklyn,
    Clarendon,
    Earlybird,
    Gingham,
    Hudson,
    Inkwell,
    Kelvin,
    Lark,
    Lofi,
    Maven,
    Mayfair,
    Moon,
    Nashville,
    Reyes,
    Rise,
    Slumber,
    Stinson,
    Toaster,
    Valencia,
    Walden,
];

fn select_filter_type(filter: &str) -> FilterType {
    let search_result = FILTER_STRINGS.iter().enumerate().find(|f| &filter == f.1);
    match search_result {
        Some((i, _)) => FILTER_TYPES[i].clone(),
        None => Valencia,
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();
}

#[wasm_bindgen]
pub fn apply_filter(img: &[u8], filter: &str) -> Box<[u8]> {
    log::debug!("Img len: {}", img.len());
    let img = ImageReader::new(Cursor::new(img))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let filter_type = select_filter_type(filter);
    let out = img.to_rgba8().apply_filter(filter_type);
    let mut bytes: Vec<u8> = Vec::new();
    out.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .unwrap();
    log::debug!("returning {} bytes", bytes.len());
    bytes.into_boxed_slice()
}
