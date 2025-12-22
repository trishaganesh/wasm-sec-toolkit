//import wasm-bindgen to expose Rust functions to JavaScript
use wasm_bindgen::prelude::*;
//the regex library for pattern matching (used to match rules against payloads)
use regex::Regex;
//serde for JSON serialization/deserialization
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rule {
    pub name: String,
}
