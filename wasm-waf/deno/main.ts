/*we instantiate the WebAssembly WAF module in Deno */
const wasm = await WebAssembly.instantiate(
  await Deno.readFile("../dist/wasm_waf.wasm"),
  {} //there are no imports required and WASM runs fully sandboxed
);

//we need to also xtract the payload inspection function from WASM exports
const { inspect_payload } = wasm.instance.exports as any;

//we then load WAF rules from a JSON file
const rules = await Deno.readTextFile("../rules/rules.json");

//here's a sample payload simulating incoming requests
const payloads = [
  "Hello world",              //the input
  "DROP TABLE users;",        //the SQL injection attempt
  "<script>alert(1)</script>" //XSS attempt
];

//we inspect each payload using the WASM WAF
for (const payload of payloads) {
  //then return true if allowed, false if blocked
  const allowed = inspect_payload(payload, rules);
//then lastly we output the inspection result
  console.log(`${payload} => ${allowed ? "Allowed" : "Blocked"}`);
}
