//the node.js file system module to read files
import fs from "fs";

//then read the compiled WebAssembly firewall module (.wasm) into a buffer
const wasmBuffer = fs.readFileSync("../dist/wasm_waf.wasm");


