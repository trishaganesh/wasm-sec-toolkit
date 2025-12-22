//import wasm-bindgen so these functions can be called from JavaScript
use wasm_bindgen::prelude::*;

//regex is used for pattern matching (email + SQL injection detection)
use regex::Regex;

/* need to validate whether a string is a properly formatted email address
exposed to JavaScript via wasm-bindgen
this returns true if the email matches the regex pattern */
#[wasm_bindgen]
