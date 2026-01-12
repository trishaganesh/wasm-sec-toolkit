//need to import the wasm-bindgen prelude so we can expose Rust functions to JavaScript
use wasm_bindgen::prelude::*;

//need to import Argon2 password hashing library and its configuration struct
use argon2::{self, Config};

/*  
Do the following:
Hash a password using Argon2id and return an encoded hash string.

This function is exposed to JavaScript via wasm-bindgen.

Parameters:
- password: The plaintext password provided by the user
- salt: A cryptographically random salt (should be unique per user)

 Returns:
- A PHC-formatted Argon2 hash string containing:
  algorithm, version, parameters, salt, and hash
*/
#[wasm_bindgen]
pub fn hash_password(password: &str, salt: &str) -> String {
    // Configure Argon2 parameters
    let config = Config {

     /*Argon2id is recommended for password hashing
        Combines resistance to GPU attacks (Argon2i)
        and side-channel resistance (Argon2d) */

     /* Memory cost in kilobytes (64 MB)
        Higher memory cost increases resistance to GPU/ASIC attacks */
        mem_cost: 65536,
