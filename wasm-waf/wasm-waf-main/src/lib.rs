//import wasm-bindgen to expose Rust functions to JavaScript
use wasm_bindgen::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rule {
    pub name: String,
}
