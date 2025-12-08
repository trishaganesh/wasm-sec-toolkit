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

    //dynamically loading the plugins
    let modules_path = "plugins/";
    //defining the folder path (plugins/) where the WebAssembly plugin files are stored
    for entry in fs::read_dir(modules_path)? {
        /*reading all entries
        and loops over each entry. ? propagates any errors if the folder can’t be read. */
        let path = entry?.path();
        //gets the full path of the current entry in the directory
        if path.extension().map(|extension_value| extension_value == "wasm").unwrap_or(false) {
            //checking if the file has a .wasm extension
            let plugin_name = path.file_name().unwrap().to_string_lossy();
            println!("Loading plugin: {}", plugin_name);

            //loading the WebAssembly module from the file at the given path
            let module = match Module::from_file(&engine, &path) {
                //if the loading succeeds, then assign the module to `module`
                Ok(loaded_module) => loaded_module,
                //if the loading fails, print an error and skip this plugin
                Err(load_error) => {
                    eprintln!("Failed to load {}: {:?}", plugin_name, e);
                    continue; //moving on to the next plugin in the loop
                }
            };

            //resetting the fuel for this plugin
            store.add_fuel(1_000_000)?; //need to check fuel units value (what is most optimal) 

            //building a fresh WASI sandbox
            let wasi = WasiCtxBuilder::new().build();
            //attaching it to the store for isolated execution
            store.set_wasi(wasi);

            //instantiating the WASM module using the linker
            let instance = match linker.instantiate(&mut store, &module) {
                //if instantiation is successful, store the resulting instance in `plugin_instance`
                Ok(plugin_instance) => plugin_instance,
    
                //if instantiation fails, capture the error in `instantiation_error`
                Err(instantiation_error) => {
                    //print an error message including the plugin name and the error details
                    eprintln!("Failed to instantiate {}: {:?}", plugin_name, instantiation_error);
                    //skip this plugin and continue to the next one
                    continue;
                }
            };

    /*
    //loading the WASM plugin
    let module = Module::from_file(&engine, "plugins/example.wasm")?;
    */

    /*
    //creating an instance of the module
    let instance = Instance::new(&mut store, &module, &[])?;
    */
            
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
