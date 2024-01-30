/*
 * This file is part of Monume's library libmonero
 *
 * Copyright (c) 2023-2024, Monume (monume.xyz)
 * All Rights Reserved
 * The code is distributed under MIT license, see LICENSE file for details.
 * Generated by Monume
 *
 */

use sha3::{Keccak256Full, Digest};
use super::{aesu::derive_key, otheru::{add_pair_u64_2, blake256_hash, groestl256_hash, jh256_hash, mul_pair_u64_2, skein256_hash, turn_to_u64, turn_to_u64_2, turn_to_u8_16, xor_pair_u64_2}};
use crate::crypt::cryptonight::aesu::{aes_round, xor};

const SCRATCHPAD_SIZE: usize = 2 * 1024 * 1024; // 2 MiB

/// Main CryptoNight function defined in: <https://web.archive.org/web/20190911221902/https://cryptonote.org/cns/cns008.txt>
/// 
/// Even though it's actually implemented in Rust for [Cuprate](https://github.com/Cuprate/cuprate), anyone can use it.
/// 
/// Example:
/// ```
/// use libmonero::crypt::cryptonight::slow_hash::cn_slow_hash;
/// 
/// let input: &str = "This is a test";
/// let output: String = cn_slow_hash(input.as_bytes());
/// assert_eq!(output, "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605".to_string());
/// ```
pub fn cn_slow_hash(input: &[u8]) -> String {
    // CryptoNight Step 1: Initialization Of Scratchpad

    //     First, the input is hashed using Keccak [KECCAK] with parameters b =
    //    1600 and c = 512. The bytes 0..31 of the Keccak final state are
    //    interpreted as an AES-256 key [AES] and expanded to 10 round keys. A
    //    scratchpad of 2097152 bytes (2 MiB) is allocated. The bytes 64..191
    //    are extracted from the Keccak final state and split into 8 blocks of
    //    16 bytes each. Each block is encrypted using the following procedure:

    //       for i = 0..9 do:
    //           block = aes_round(block, round_keys[i])

    //    Where aes_round function performs a round of AES encryption, which
    //    means that SubBytes, ShiftRows and MixColumns steps are performed on
    //    the block, and the result is XORed with the round key. Note that
    //    unlike in the AES encryption algorithm, the first and the last rounds
    //    are not special. The resulting blocks are written into the first 128
    //    bytes of the scratchpad. Then, these blocks are encrypted again in
    //    the same way, and the result is written into the second 128 bytes of
    //    the scratchpad. Each time 128 bytes are written, they represent the
    //    result of the encryption of the previously written 128 bytes. The
    //    process is repeated until the scratchpad is fully initialized.

    // Step 1A: Initialize the scratchpad with empty data
    let mut scratchpad = [0u8; SCRATCHPAD_SIZE];

    // Step 1B: Use Keccak256Full to hash the input
    let mut keccak_hash = [0u8; 200];
    let mut hasher = Keccak256Full::new();
    hasher.update(input);
    keccak_hash.copy_from_slice(&hasher.finalize());

    // Step 1C: Use the first 32 bytes of the Keccak hash as an AES-256 key and expand it into 10 round keys
    let aes_key = &keccak_hash[0..32];
    let round_keys = derive_key(aes_key);

    // Step 1D: Use bytes 64..191 of the Keccak hash as 8 blocks of 16 bytes each
    let mut blocks = [0u8; 128];
    blocks.copy_from_slice(&keccak_hash[64..192]);

    // Step 1E: Loop until scratchpad is fully initialized
    for scratchpad_chunk in scratchpad.chunks_exact_mut(blocks.len()) {
        for block in blocks.chunks_exact_mut(16) {
            for key in round_keys.chunks_exact(16) {
                aes_round(block, key);
            }
        }

        scratchpad_chunk.copy_from_slice(&blocks);
    }

    // Cryptonight Step 2: Memory-hard Loop

    // Prior to the main loop, bytes 0..31 and 32..63 of the Keccak state
    // are XORed, and the resulting 32 bytes are used to initialize
    // variables a and b, 16 bytes each. These variables are used in the
    // main loop. The main loop is iterated 524,288 times. When a 16-byte
    // value needs to be converted into an address in the scratchpad, it is
    // interpreted as a little-endian integer, and the 21 low-order bits are
    // used as a byte index. However, the 4 low-order bits of the index are
    // cleared to ensure the 16-byte alignment. The data is read from and
    // written to the scratchpad in 16-byte blocks. Each iteration can be
    // expressed with the following pseudo-code:

    //     scratchpad_address = to_scratchpad_address(a)
    //     scratchpad[scratchpad_address] = aes_round(scratchpad
    //     [scratchpad_address], a)
    //     b, scratchpad[scratchpad_address] = scratchpad[scratchpad_address],
    //     b xor scratchpad[scratchpad_address]
    //     scratchpad_address = to_scratchpad_address(b)
    //     a = 8byte_add(a, 8byte_mul(b, scratchpad[scratchpad_address]))
    //     a, scratchpad[scratchpad_address] = a xor
    //     scratchpad[scratchpad_address], a

    // Where, the 8byte_add function represents each of the arguments as a
    // pair of 64-bit little-endian values and adds them together,
    // component-wise, modulo 2^64. The result is converted back into 16
    // bytes.

    // The 8byte_mul function, however, uses only the first 8 bytes of each
    // argument, which are interpreted as unsigned 64-bit little-endian
    // integers and multiplied together. The result is converted into 16
    // bytes, and finally the two 8-byte halves of the result are swapped.

    // Step 2A: Turn [u8; 200] into [[u64; 2]; 131072] for easier access
    let mut sp_u64_2 = [[0u64; 2]; 131072];
    for (i, sp_u64_2_chunk) in sp_u64_2.iter_mut().enumerate() {
        let u64_slice = unsafe {
            std::slice::from_raw_parts(scratchpad[i * 16..(i + 1) * 16].as_ptr() as *const u64, 2)
        };
        sp_u64_2_chunk.copy_from_slice(u64_slice);
    }

    // Step 2B: Get a and b as described above
    let a_1: u64 = turn_to_u64(&keccak_hash[0..8]) ^ turn_to_u64(&keccak_hash[32..40]);
    let a_2: u64 = turn_to_u64(&keccak_hash[8..16]) ^ turn_to_u64(&keccak_hash[40..48]);
    let b_1: u64 = turn_to_u64(&keccak_hash[16..24]) ^ turn_to_u64(&keccak_hash[48..56]);
    let b_2: u64 = turn_to_u64(&keccak_hash[24..32]) ^ turn_to_u64(&keccak_hash[56..64]);
    let mut a: [u64; 2] = [a_1, a_2];
    let mut b: [u64; 2] = [b_1, b_2];

    // Step 2C: Loop 524,288 times
    for _ in 0..524_288 {
        // Step 2C1: First Transfer
        let addr: usize = (a[0] & 0x1F_FFF0) as usize / 16;
        let block = &mut turn_to_u8_16(sp_u64_2[addr]);
        aes_round(block, &turn_to_u8_16(a));
        sp_u64_2[addr] = turn_to_u64_2(*block);
        let tmp = b;
        b = sp_u64_2[addr];
        let man = xor_pair_u64_2(sp_u64_2[addr], tmp);
        sp_u64_2[addr] = man;

        // Step 2C2: Second Transfer
        let addr: usize = (b[0] & 0x1F_FFF0) as usize / 16;
        let tmp = add_pair_u64_2(a, mul_pair_u64_2(b, sp_u64_2[addr]));
        a = xor_pair_u64_2(sp_u64_2[addr], tmp);
        sp_u64_2[addr] = tmp;
    }

    // Step 2D: Turn [[u64; 2]; 131072] into [u8; 2097152] for easier access
    for (i, sp_u64_2_chunk) in sp_u64_2.iter().enumerate() {
        let u8_slice = unsafe {
            std::slice::from_raw_parts(sp_u64_2_chunk.as_ptr() as *const u8, 16)
        };
        scratchpad[i * 16..(i + 1) * 16].copy_from_slice(u8_slice);
    }

    // Cryptonight Step 3: Result Calculation

    // After the memory-hard part, bytes 32..63 from the Keccak state are
    // expanded into 10 AES round keys in the same manner as in the first
    // part.

    // Bytes 64..191 are extracted from the Keccak state and XORed with the
    // first 128 bytes of the scratchpad. Then the result is encrypted in
    // the same manner as in the first part, but using the new keys. The
    // result is XORed with the second 128 bytes from the scratchpad,
    // encrypted again, and so on. 

    // After XORing with the last 128 bytes of the scratchpad, the result is
    // encrypted the last time, and then the bytes 64..191 in the Keccak
    // state are replaced with the result. Then, the Keccak state is passed
    // through Keccak-f (the Keccak permutation) with b = 1600. 

    // Then, the 2 low-order bits of the first byte of the state are used to
    // select a hash function: 0=BLAKE-256 [BLAKE], 1=Groestl-256 [GROESTL],
    // 2=JH-256 [JH], and 3=Skein-256 [SKEIN]. The chosen hash function is
    // then applied to the Keccak state, and the resulting hash is the
    // output of CryptoNight.

    // Step 3A: Encrypt the scratchpad with the new keys
    let round_keys_buffer = derive_key(&keccak_hash[32..64]);
    let final_block = &mut keccak_hash[64..192];
    for scratchpad_chunk in scratchpad.chunks_exact(128) {
        xor(final_block, scratchpad_chunk);
        for block in final_block.chunks_exact_mut(16) {
            for key in round_keys_buffer.chunks_exact(16) {
                aes_round(block, key);
            }
        }
    }

    // Step 3B: Turn keccak_hash to [u64; 25] and pass it through Keccak-f, then turn it back to [u8; 200]
    let mut keccak_state = [0u64; 25];
    for (index, chunk) in keccak_hash.chunks_exact(8).enumerate() {
        keccak_state[index] = u64::from_le_bytes(chunk.try_into().unwrap());
    }
    tiny_keccak::keccakf(&mut keccak_state);
    for (index, chunk) in keccak_state.iter().enumerate() {
        keccak_hash[index * 8..(index + 1) * 8].copy_from_slice(&chunk.to_le_bytes());
    }

    // Step 3C: Use the first byte of the Keccak state to select a hash function
    let hash_function = keccak_hash[0] & 0x03;
    let final_byte = match hash_function {
        0 => blake256_hash(keccak_hash),
        1 => groestl256_hash(keccak_hash),
        2 => jh256_hash(keccak_hash),
        3 => skein256_hash(keccak_hash),
        x => unreachable!("Hash function {} not implemented", x),
    };
    
    // Step 3D: Turn the final byte into a hex string and return
    let mut final_hex = String::new();
    for byte in final_byte.iter() {
        final_hex.push_str(&format!("{:02x}", byte));
    }
    final_hex
}