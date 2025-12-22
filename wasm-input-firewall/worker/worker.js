//more info at: https://developers.cloudflare.com/workers/wrangler/configuration/

export default {
  /* workers entry point
  and this function runs for every incoming HTTP request */
  async fetch(request, env) {

    /* instantiate the WASM firewall module
    env.FIREWALL_WASM is a bound WASM binary provided via wrangler.toml */
    const wasm = await WebAssembly.instantiate(
      env.FIREWALL_WASM,
      {} //there are no imports required; sandboxed by default
    );

    //to extract the SQL injection detection function
    const { 
    detect_sql_injection 
    } = wasm.instance.exports;

    /*and to read the request body as plain text
    and this allows inspection of POST/PUT payloads */
    const body = await request.text();

    //if SQL injection patterns are detected, block the request
    if (detect_sql_injection(body)) {
      return new Response(
        "Blocked by WASM Firewall",
        { status: 403 } //forbidden!
      );
    }

    //iff no threats are found, allow the request to proceed
    return new Response("Request allowed");
  }
};
