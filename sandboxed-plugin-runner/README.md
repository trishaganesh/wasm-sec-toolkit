# WASM Sandboxed Plugin Runner

## Goal
Safely execute untrusted WebAssembly (WASM) plugins inside a controlled sandbox while protecting the host system from malicious or faulty code.

## How
- Using **Wasmtime** to execute `.wasm` modules
- Dynamically load plugins from `plugins/` folder
- Restricting the capabilities (no network, the memory limits can be configured, the CPU timeouts can be possible)

## Security
- **Capability-based security**: Plugins only have access to the functions and resources explicitly granted by the host. There is no ambient authority—if a capability is not provided, it cannot be used.
- **Deterministic execution**: WebAssembly enforces a well-defined execution model with no undefined behavior, raw system calls, or arbitrary memory access, making plugin behavior predictable and auditable.
- **Resource limits**: The runtime enforces:

          CPU limits using Wasmtime fuel metering to prevent infinite loops or denial-of-service attacks

          Memory limits by configuring maximum linear memory per plugin

          Execution time measurement to detect slow or abusive plugins

## Usage
Compile Rust project:
```bash
cargo build --release

Add plugins:

Place compiled .wasm files into the plugins/ directory
Each plugin must export a run function with no parameters and no return value
