use base64::prelude::*;
use image::load_from_memory;
use image::ImageFormat::Png;
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    let base64_to_vector = BASE64_STANDARD.decode(encoded_file).unwrap();
    let mut img = load_from_memory(&base64_to_vector).unwrap();

    img = img.grayscale();

    let mut buffer = Cursor::new(vec![]);
    img.write_to(&mut buffer, Png).unwrap();

    let encoded_img = BASE64_STANDARD.encode(buffer.into_inner());
    format!("data:image/png;base64,{}", encoded_img)
}
