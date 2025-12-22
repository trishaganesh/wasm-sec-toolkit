//import wasm-bindgen so these functions can be called from JavaScript
use wasm_bindgen::prelude::*;

//regex is used for pattern matching (email + SQL injection detection)
use regex::Regex;

/* need to validate whether a string is a properly formatted email address
exposed to JavaScript via wasm-bindgen
this returns true if the email matches the regex pattern */
#[wasm_bindgen]
pub fn validate_email(email: &str) -> bool {
    /*
 - Allows alphanumeric characters and common symbols
- Requires an '@' symbol
- Requires a valid domain and TLD (2+ characters) */
    let reg = Regex::new(
        r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$"
    ).unwrap(); //this is a safe unwrap: regex is static and known-valid

    //need to check if the input email matches the pattern
    reg.is_match(email)
}
