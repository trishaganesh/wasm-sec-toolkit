#ifndef AES_HPP
#define AES_HPP

#include <cstddef>
#include <cstdint>

/*AES class (AES-128)
 the input and key must be 16 bytes, the Output buffer must have enough space
 */
class AES {
public:
    //this encrypts input using key, writes result to output
    static void encrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length);

    //this decrypts input using key, writes result to output
    static void decrypt(uint8_t* input, uint8_t* key, uint8_t* output, size_t length);
};

#endif 
