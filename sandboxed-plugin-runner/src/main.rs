use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::{
    fs,
    path::PathBuf,
    sync::mpsc::channel,
    time::{Duration, Instant},
};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use std::thread;

fn main() -> anyhow::Result<()> {
    println!("WASM Sandboxed Plugin Runner");

    //configuring the Wasmtime engine with the fuel consumption
    let mut config = Config::new();
    // this will enable fuel-based CPU limiting
    config.consume_fuel(true); 
    //add memory for each plugin: 16MB
    config.static_memory_maximum_size(16 * 1024 * 1024);
    
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

    //a folder to watch for the plugins
    let plugin_folder = PathBuf::from("plugins/");
    //checking to make sure it exists
    fs::create_dir_all(&plugin_folder)?; 
    //channel for watching the folder
    let (tx, rx) = channel();

    //watch the folder for changes
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(&plugin_folder, RecursiveMode::NonRecursive)?;
    
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

            
            /*trying to get the exported function named "run" from the plugin instance
             expect it to have no parameters and no return value: <(), ()> */
            if let Ok(run_func) = instance.get_typed_func::<(), (), _>(&mut store, "run") {
                //recording the current time so we can measure execution duration later
                let execution_start = Instant::now();
                //executing the "run" function inside the sandboxed store
                match run_func.call(&mut store, ()) {
                     Ok(_) => {
                        //calculating how long the plugin took to run
                        let elapsed_time = start.elapsed();
                        //checking how much CPU "fuel" this plugin consumed
                        let fuel_used = store.fuel_consumed().unwrap_or(0);
                        //reporting success, timing, and fuel usage
                        println!(
                            "Plugin {} executed successfully in {:?} using {} fuel units",
                            plugin_name, elapsed_time, fuel_used
                        );
                    }
                    //if execution failed, print the error
                    Err(e) => eprintln!("Plugin {} execution failed: {:?}", plugin_name, e),
                }
            //if the module doesn't export a "run" function, warn the user
            } else {
                eprintln!("Plugin {} does not export 'run'", plugin_name);
            }
        }
    }
    //when all plugin files have been processed, print a final summary
    println!("All plugins executed");
    //return success from main()
    Ok(())
}
