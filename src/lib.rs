use wasm_bindgen::prelude::*;

#[macro_use]
mod console;

#[wasm_bindgen]
pub fn answer() -> i32 {
    42
}
