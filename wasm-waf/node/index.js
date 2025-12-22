//the node.js file system module to read files
import fs from "fs";

//then read the compiled WebAssembly firewall module (.wasm) into a buffer
const wasmBuffer = fs.readFileSync("../dist/wasm_waf.wasm");

/* instantiate the WebAssembly module
empty imports object because the module is self-contained */
const wasm = await WebAssembly.instantiate(wasmBuffer, {});

/*destructure the exported function inspect_payload from WASM
this function checks payloads against a JSON set of rules */
const { 
  inspect_payload 
} = wasm.instance.exports;

//load security rules from a JSON file and each rule should have name and pattern fields
const rules = fs.readFileSync("../rules/rules.json", "utf8");

//example incoming request/payloads to inspect
const payloads = [
  "Hello world",             //the safe payload
  "DROP TABLE users;",       //the SQL injection
  "<script>alert(1)</script>" //XSS attempt
];

//loop through each payload and check if it is allowed
for (const payload of payloads) {
  //call WASM function: returns true if allowed, false if blocked
  const allowed = inspect_payload(payload, rules);
