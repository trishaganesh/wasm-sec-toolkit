#ifndef AES_H
#define AES_H

#include <stdint.h>
#include <stddef.h>

/*the AES-128 encryption and decryption functions
the input and key must be 16 bytes. output must have enough space */

void aes_encrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length);
void aes_decrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length);

#endif 

