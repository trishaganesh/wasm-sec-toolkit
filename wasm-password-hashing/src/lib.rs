//need to import the wasm-bindgen prelude so we can expose Rust functions to JavaScript
use wasm_bindgen::prelude::*;

//need to import Argon2 password hashing library and its configuration struct
use argon2::{self, Config};
