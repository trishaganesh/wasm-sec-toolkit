//the node.js file system module used to load the compiled WebAssembly binary
import fs from "fs";

/* here we read the compiled WebAssembly firewall module into a buffer
and this .wasm file is produced by wasm-pack or cargo build */
const wasmBuffer = fs.readFileSync("../dist/wasm_firewall.wasm");

/* we instantiate the WebAssembly module
there are no imports are required because the module is self-contained */
const wasm = await WebAssembly.instantiate(wasmBuffer, {});

/* the destructure exported WebAssembly functions
these functions were exposed using #[wasm_bindgen] in Rust */
const {
  validate_email,       //this validates email format using regex
  sanitize_url,         //this will remove dangerous URL substrings
  detect_sql_injection  //this will detects common SQL injection patterns
} = wasm.instance.exports;

/* 
some sample test cases
*/

//the test email validation
console.log(
  "Email valid:",
  validate_email("test@example.com") //expected: true
);

//the test URL sanitization against JavaScript injection
console.log(
  "Sanitized URL:",
  sanitize_url("javascript:alert(1)") //expected: "alert(1)"
);

//the test SQL injection detection
console.log(
  "SQL Injection detected:",
  detect_sql_injection("DROP TABLE users;") //expected: true
);
