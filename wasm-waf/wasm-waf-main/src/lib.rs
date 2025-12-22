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

/* then we Load a JSON string of rules and return them as a JsValue
the  this function is exposed to JavaScript via wasm-bindgen
json_rules: JSON array of objects, each with name and pattern
this returns: JsValue (serializable to JavaScript array of rules) */
#[wasm_bindgen]
pub fn load_rules(json_rules: &str) -> JsValue {
    //then deserialize JSON string into Vec<Rule>
   
