/*we instantiate the WebAssembly WAF module in Deno */
const wasm = await WebAssembly.instantiate(
  await Deno.readFile("../dist/wasm_waf.wasm"),
  {} //there are no imports required and WASM runs fully sandboxed
);

//we need to also xtract the payload inspection function from WASM exports
const { inspect_payload } = wasm.instance.exports as any;

//we then load WAF rules from a JSON file
const rules = await Deno.readTextFile("../rules/rules.json");
