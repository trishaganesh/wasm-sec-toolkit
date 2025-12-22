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

/* we need to sanitize URLs by removing potentially dangerous substrings
this helps prevent XSS and script injection attacks
before the URL reaches backend services */
#[wasm_bindgen]
pub fn sanitize_url(url: &str) -> String {
    // Common attack vectors and unsafe characters
    let forbidden = [
        "javascript:", //the JS execution URLs
        "data:",       //the data URIs (can embed scripts)
        "<", ">",      //the HTML tags
        "\"", "'",     //the attribute injection
    ];

//but then we need to create a mutable copy of the input
    let mut clean = url.to_string();

    //then we remove all forbidden patterns from the URL
    for f in forbidden {
        clean = clean.replace(f, "");
    }

    //lastly, we return sanitized URL
    clean
}

/* but we detect common SQL injection patterns in user input
returns true if suspicious SQL patterns are found
then intended for early input filtering before backend processing */
#[wasm_bindgen]
pub fn detect_sql_injection(input: &str) -> bool {
    // List of known SQL injection indicators
    let patterns = [
        r"(?i)select\s+.*from", //SELECT ... FROM (case-insensitive)
        r"(?i)union\s+select",  //UNION SELECT attacks
        r"(?i)drop\s+table",    //DROP TABLE attempts
        r"--",                  //SQL comment injection
        r";",                   //this is statement chaining
        r"' OR '1'='1",         //the tautology attack
    ];

    //we need to check each pattern against the input
    for p in patterns {
        let reg = Regex::new(p).unwrap(); //then the static patterns are safe to unwrap
        if reg.is_match(input) {
            //in case suspicious SQL detected
            return true;
        }
    }

    //there are no injection patterns found
    false
}
