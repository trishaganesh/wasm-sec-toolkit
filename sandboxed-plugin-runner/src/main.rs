use wasmtime::*;

fn main() -> anyhow::Result<()> {
    println!("=== WASM Sandboxed Plugin Runner ===");

    //creating the engine and store
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

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
