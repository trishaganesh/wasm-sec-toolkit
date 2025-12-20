/* this Loads a WebAssembly module from the given path
 the URL or relative path to the .wasm file
 the exported functions and memory from the WASM module
 */
export async function loadWasmModule(path) {
    //then fetch the .wasm file
    const wasm = await fetch(path);

    //then convert response to an ArrayBuffer
    const bytes = await wasm.arrayBuffer();

    //then instantiate the WebAssembly module
    const { instance } = await WebAssembly.instantiate(bytes, {});

    //then return the exported functions and memory
    return instance.exports;
}
