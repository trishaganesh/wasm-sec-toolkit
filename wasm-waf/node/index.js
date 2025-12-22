//the node.js file system module to read files
import fs from "fs";

//then read the compiled WebAssembly firewall module (.wasm) into a buffer
const wasmBuffer = fs.readFileSync("../dist/wasm_waf.wasm");

/* instantiate the WebAssembly module
empty imports object because the module is self-contained */
const wasm = await WebAssembly.instantiate(wasmBuffer, {});

