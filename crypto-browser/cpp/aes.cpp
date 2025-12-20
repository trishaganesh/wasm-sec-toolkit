#include "aes.hpp"

/*an XOR-based placeholder for AES
each byte of input is XORed with the key, repeating every 16 bytes */

void AES::encrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length) {
    for (size_t i = 0; i < length; i++) {
        output[i] = input[i] ^ key[i % 16];
    }
}

void AES::decrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length) {
    //this XOR decryption is also identical to encryption
    for (size_t i = 0; i < length; i++) {
        output[i] = input[i] ^ key[i % 16];
    }
}
