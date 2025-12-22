/*we instantiate the WebAssembly WAF module in Deno */
const wasm = await WebAssembly.instantiate(
  await Deno.readFile("../dist/wasm_waf.wasm"),
  {} //there are no imports required and WASM runs fully sandboxed
);
