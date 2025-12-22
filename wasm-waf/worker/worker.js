export default {
  //this is the Workers entry point
 function: runs for every incoming HTTP request */
  async fetch(request, env) {

    /*we Instantiate the WASM WAF module from the environment binding*/
    const wasm = await WebAssembly.instantiate(env.WAF_WASM, {});

    /*then extract the `inspect_payload` function from WASM exports */
    const { inspect_payload } = wasm.instance.exports;

    //then read the request body as plain text (supports POST/PUT payload inspection)
    const payload = await request.text();

    /*then load security rules from a Cloudflare KV or static binding */
    const rules_json = await env.RULES_JSON.text();

    /* then inspect the payload using the WASM WAF
    returns true if allowed, false if blocked */
    const allowed = inspect_payload(payload, rules_json);

    //then block request if any rule matches
    if (!allowed) {
      return new Response("Blocked by WASM WAF", { status: 403 });
    }

    //lasyl, allow request if no rules match
    return new Response("Request allowed");
  }
};
