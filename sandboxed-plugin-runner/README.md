# WASM Sandboxed Plugin Runner

## Goal
Safely execute untrusted WebAssembly (WASM) plugins inside a controlled sandbox while protecting the host system from malicious or faulty code.

## How
- Using **Wasmtime** to execute `.wasm` modules
- Dynamically load plugins from `plugins/` folder
- Restricting the capabilities (no network, the memory limits can be configured, the CPU timeouts can be possible)

## Security
- **Capability-based security**: Plugins only have access to the functions and resources explicitly granted by the host. There is no ambient authority—if a capability is not provided, it cannot be used.
- **Deterministic execution**: There's no undefined behavior or arbitrary system calls
- **Resource limits**: Infinite loops or excessive memory usage are prevented

## Usage
Compile Rust project:
```bash
cargo build --release
