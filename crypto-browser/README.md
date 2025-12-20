# Crypto Browser

### Emscripten Compilation Commands 
C command: 
emcc c/aes.c -O3 -s WASM=1 -s EXPORTED_FUNCTIONS="['_aes_encrypt','_aes_decrypt','_malloc','_free']" -o wasm/aes_c.js

C++ command: 
em++ cpp/aes.cpp -O3 -s WASM=1 -s EXPORTED_FUNCTIONS="['_AES_encrypt','_AES_decrypt','_malloc','_free']" -o wasm/aes_cpp.js
