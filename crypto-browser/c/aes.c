#include "aes.h"
#include <string.h>
#include <stdio.h>

/* a Simple XOR-based placeholder for AES (for demonstration only)
where each byte of input is XORed with the key (repeating every 16 bytes) */

void aes_encrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length) {
    for (size_t i = 0; i < length; i++) {
        output[i] = input[i] ^ key[i % 16];
    }
}

void aes_decrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length) {
    //here the XOR decryption is identical to encryption
    for (size_t i = 0; i < length; i++) {
        output[i] = input[i] ^ key[i % 16];
    }
}
