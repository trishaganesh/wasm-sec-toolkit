use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use std::fs;
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    println!("WASM Sandboxed Plugin Runner");

    //configuring the Wasmtime engine with the fuel consumption
    let mut config = Config::new();
    // this will enable fuel-based CPU limiting
    config.consume_fuel(true); 
    
    //creating the engine and store
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    //creating a new linker associated with the Wasmtime engine
    let mut linker = Linker::new(&engine);

    /*defining a host function "host_print" in the "env" module allowing the
     Wasm module to call this function */
    linker.func_wrap("env", "host_print", |msg_ptr: i32, msg_len: i32, memory: Memory| {
        //getting the WebAssembly module's linear memory
        let data = memory.data(&store);
        //extracting the bytes corresponding to the message from memory
        let bytes = &data[msg_ptr as usize..(msg_ptr + msg_len) as usize];
        //converting the bytes to a UTF-8 string, if possible
        if let Ok(msg) = std::str::from_utf8(bytes) {
            println!("[PLUGIN]: {}", msg);
        }
    })?;
    
    //loading the WASM plugin
    let module = Module::from_file(&engine, "plugins/example.wasm")?;

    //creating an instance of the module
    let instance = Instance::new(&mut store, &module, &[])?;

    //calling the exported function 'run' if it exists
    if let Some(run_func) = instance.get_typed_func::<(), (), _>(&mut store, "run").ok() {
        println!("Running plugin...");
        run_func.call(&mut store, ())?;
        println!("Plugin finished execution!");
    } else {
        println!("No 'run' function found in plugin.");
    }
    Ok(())
}
