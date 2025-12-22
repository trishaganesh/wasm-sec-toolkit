/** this will essentially load and instantiate the WebAssembly firewall module in Deno
Deno.readFile returns a Uint8Array containing the .wasm binary 
Some documentation / info about deno I used can be found here: https://github.com/denoland/deno
*/

const wasm = await WebAssembly.instantiate(
  await Deno.readFile("../dist/wasm_firewall.wasm"),
  {} //there are no imports needed; module is fully self-contained
);

/**this destructure exported WebAssembly functions
and as any is used because WebAssembly exports are untyped at runtime */
const {
  validate_email,       //the email format validation using regex
  sanitize_url,         //URL sanitization to prevent XSS vectors
  detect_sql_injection  //the detection of common SQL injection patterns
} = wasm.instance.exports as any;

//some more test cases

//to validate an email address
console.log(
  validate_email("hello@deno.land") //expected: true
);

//to sanitize a potentially malicious URL
console.log(
  sanitize_url("javascript:evil()") //expected: "evil()"
);

//to detect SQL injection attempts
console.log(
  detect_sql_injection("SELECT * FROM users") //expected: true
);
