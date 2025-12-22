//import wasm-bindgen to expose Rust functions to JavaScript
use wasm_bindgen::prelude::*;
//the regex library for pattern matching (used to match rules against payloads)
use regex::Regex;
//serde for JSON serialization/deserialization
use serde::Deserialize;

// a Struct representing a security rule
/* first off the name: human-readable identifier for the rule
then pattern: regex pattern to match against payloads */
#[derive(Deserialize)]
pub struct Rule {
    pub name: String,
    pub pattern: String,
}
