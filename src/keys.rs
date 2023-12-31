/*
 * This file is part of Monume's library libmonero
 *
 * Copyright (c) 2023-2024, Monume (monume.xyz)
 * All Rights Reserved
 * The code is distributed under MIT license, see LICENSE file for details.
 * Generated by Monume
 *
 */

//! # Keys
//!
//! This module is for everything related to keys, such as generating seeds, deriving keys from seeds, deriving public keys from private keys, and deriving addresses from public keys.

use crate::crypt::ed25519::sc_reduce32;
use crate::wordsets::{WordsetOriginal, WORDSETSORIGINAL};
use crc32fast::Hasher;
use curve25519_dalek::{constants::ED25519_BASEPOINT_TABLE, EdwardsPoint, Scalar};
use rand::Rng;
use sha3::{Digest, Keccak256};
use std::ops::Mul;

/// Returns cryptographically secure random element of the given array
fn secure_random_element<'x>(array: &'x [&'x str]) -> &'x str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..array.len());
    array[random_index]
}

/// Calculates CRC32 checksum index for given array (probably the seed)
fn get_checksum_index(array: &[&str], prefix_length: usize) -> usize {
    let mut trimmed_words: String = String::new();
    for word in array {
        trimmed_words.push_str(&word[0..prefix_length]);
    }
    let mut hasher = Hasher::new();
    hasher.update(trimmed_words.as_bytes());
    usize::try_from(hasher.finalize()).unwrap() % array.len()
}

/// Generates a cryptographically secure 1626-type (25-word) seed for given language
fn generate_original_seed(language: &str) -> Vec<&str> {
    // Check if language is supported
    if !WORDSETSORIGINAL.iter().any(|x| x.name == language) {
        panic!("Language not found");
    }
    // Generate seed
    let mut seed: Vec<&str> = Vec::new();
    let mut prefix_len: usize = 3;
    for wordset in WORDSETSORIGINAL.iter() {
        if wordset.name == language {
            prefix_len = wordset.prefix_len;
            for _ in 0..24 {
                let word = secure_random_element(&wordset.words[..]);
                seed.push(word);
            }
            break;
        } else {
            continue;
        }
    }
    // Add checksum word
    let checksum_index = get_checksum_index(&seed, prefix_len);
    seed.push(seed[checksum_index]);
    // Finally, return the seed
    seed
}

/// Generates a cryptographically secure 1626-type (13-word) seed for given language
fn generate_mymonero_seed(language: &str) -> Vec<&str> {
    // Check if language is supported
    if !WORDSETSORIGINAL.iter().any(|x| x.name == language) {
        panic!("Language not found");
    }
    // Generate seed
    let mut seed: Vec<&str> = Vec::new();
    let mut prefix_len: usize = 3;
    for wordset in WORDSETSORIGINAL.iter() {
        if wordset.name == language {
            prefix_len = wordset.prefix_len;
            for _ in 0..12 {
                let word = secure_random_element(&wordset.words[..]);
                seed.push(word);
            }
            break;
        } else {
            continue;
        }
    }
    // Add checksum word
    let checksum_index = get_checksum_index(&seed, prefix_len);
    seed.push(seed[checksum_index]);
    // Finally, return the seed
    seed
}

/// Creates a cryptographically secure seed of given type and language
pub fn generate_seed(language: &str, seed_type: &str) -> Vec<String> {
    let seed;
    match seed_type {
        "original" => seed = generate_original_seed(language),
        "mymonero" => seed = generate_mymonero_seed(language),
        "polyseed" => panic!("Polyseed not yet implemented yet"),
        _ => panic!("Invalid seed type"),
    }
    let mut seed_string: Vec<String> = Vec::new();
    for word in seed {
        seed_string.push(word.to_string());
    }
    seed_string
}

/// Swaps endianness of a 4-byte string
fn swap_endian_4_byte(s: &str) -> String {
    format!("{}{}{}{}", &s[6..8], &s[4..6], &s[2..4], &s[0..2])
}

/// Derives hex seed from given mnemonic seed
pub fn derive_hex_seed(mut mnemonic_seed: Vec<String>) -> String {
    // Find the wordset for the given seed
    let mut the_wordset = &WordsetOriginal {
        name: "x",
        prefix_len: 0,
        words: [""; 1626],
    };
    for wordset in WORDSETSORIGINAL.iter() {
        if mnemonic_seed
            .iter()
            .all(|elem| wordset.words.contains(&elem.as_str()))
        {
            the_wordset = wordset;
            break;
        }
    }
    if the_wordset.name == "x" {
        panic!("Wordset could not be found for given seed, please check your seed");
    }

    // Remove checksum word
    if the_wordset.prefix_len > 0 {
        mnemonic_seed.pop();
    }

    // Get a vector of truncated words
    let mut trunc_words: Vec<&str> = Vec::new();
    for word in the_wordset.words.iter() {
        trunc_words.push(&word[..the_wordset.prefix_len]);
    }
    if trunc_words.is_empty() {
        panic!("Something went wrong when decoding your private key, please try again");
    }

    // Derive hex seed
    let mut hex_seed = String::new();
    let wordset_len: usize = the_wordset.words.len();
    for i in (0..mnemonic_seed.len()).step_by(3) {
        let (w1, w2, w3): (usize, usize, usize);
        if the_wordset.prefix_len == 0 {
            w1 = the_wordset
                .words
                .iter()
                .position(|&x| x == mnemonic_seed[i])
                .unwrap_or_else(|| panic!("Invalid word in seed, please check your seed"));
            w2 = the_wordset
                .words
                .iter()
                .position(|&x| x == mnemonic_seed[i + 1])
                .unwrap_or_else(|| panic!("Invalid word in seed, please check your seed"));
            w3 = the_wordset
                .words
                .iter()
                .position(|&x| x == mnemonic_seed[i + 2])
                .unwrap_or_else(|| panic!("Invalid word in seed, please check your seed"));
        } else {
            w1 = trunc_words
                .iter()
                .position(|&x| x.starts_with(&mnemonic_seed[i][..the_wordset.prefix_len]))
                .unwrap_or_else(|| panic!("Invalid word in seed, please check your seed"));
            w2 = trunc_words
                .iter()
                .position(|&x| x.starts_with(&mnemonic_seed[i + 1][..the_wordset.prefix_len]))
                .unwrap_or_else(|| panic!("Invalid word in seed, please check your seed"));
            w3 = trunc_words
                .iter()
                .position(|&x| x.starts_with(&mnemonic_seed[i + 2][..the_wordset.prefix_len]))
                .unwrap_or_else(|| panic!("Invalid word in seed, please check your seed"));
        }

        let x = w1
            + wordset_len * (((wordset_len - w1) + w2) % wordset_len)
            + wordset_len * wordset_len * (((wordset_len - w2) + w3) % wordset_len);
        if x % wordset_len != w1 {
            panic!("Something went wrong when decoding your private key, please try again");
        }

        hex_seed += &swap_endian_4_byte(&format!("{:08x}", x));
    }

    hex_seed
}

/// Derives private keys for original (25-word) (64-byte hex) type seeds
fn derive_original_priv_keys(hex_seed: String) -> Vec<String> {
    // Turn hex seed into bytes
    let hex_bytes = hex::decode(hex_seed).unwrap();
    let mut hex_bytes_array = [0u8; 32];
    hex_bytes_array.copy_from_slice(&hex_bytes);
    // Pass bytes through sc_reduce32 function to get private spend key
    sc_reduce32(&mut hex_bytes_array);
    let mut priv_spend_key = String::new();
    for i in (0..hex_bytes_array.len()).step_by(32) {
        let mut priv_key = String::new();
        for j in i..i + 32 {
            priv_key.push_str(&format!("{:02x}", hex_bytes_array[j]));
        }
        priv_spend_key.push_str(&priv_key);
    }
    // Turn private spend key into bytes and pass through Keccak256 function
    let priv_spend_key_bytes = hex::decode(priv_spend_key.clone()).unwrap();
    let priv_view_key_bytes = Keccak256::digest(priv_spend_key_bytes);
    let mut priv_view_key_array = [0u8; 32];
    priv_view_key_array.copy_from_slice(&priv_view_key_bytes);
    // Pass bytes through sc_reduce32 function to get private view key
    sc_reduce32(&mut priv_view_key_array as &mut [u8; 32]);
    let mut priv_view_key = String::new();
    for i in (0..priv_view_key_array.len()).step_by(32) {
        let mut priv_key = String::new();
        for j in i..i + 32 {
            priv_key.push_str(&format!("{:02x}", priv_view_key_array[j]));
        }
        priv_view_key.push_str(&priv_key);
    }
    // Finally, return the keys
    vec![priv_spend_key, priv_view_key]
}

/// Derives private keys for MyMonero (13-word) (32-byte hex) type seeds
fn derive_mymonero_priv_keys(hex_seed: String) -> Vec<String> {
    // Keccak and sc_reduce32 to get private spend key
    let hex_bytes = hex::decode(hex_seed).unwrap();
    let priv_spend_key_bytes = Keccak256::digest(&hex_bytes);
    let mut priv_spend_key_array = [0u8; 32];
    priv_spend_key_array.copy_from_slice(&priv_spend_key_bytes);
    sc_reduce32(&mut priv_spend_key_array as &mut [u8; 32]);
    let mut priv_spend_key = String::new();
    for i in (0..priv_spend_key_array.len()).step_by(32) {
        let mut priv_key = String::new();
        for j in i..i + 32 {
            priv_key.push_str(&format!("{:02x}", priv_spend_key_array[j]));
        }
        priv_spend_key.push_str(&priv_key);
    }
    // Double Keccak and sc_reduce32 of hex_seed to get private view key
    let priv_view_key_bytes = Keccak256::digest(&hex_bytes);
    let mut priv_view_key_array = [0u8; 32];
    priv_view_key_array.copy_from_slice(&priv_view_key_bytes);
    // Keccak again
    let priv_view_key_bytes = Keccak256::digest(priv_view_key_array);
    priv_view_key_array.copy_from_slice(&priv_view_key_bytes);
    // sc_reduce32
    sc_reduce32(&mut priv_view_key_array as &mut [u8; 32]);
    let mut priv_view_key = String::new();
    for i in (0..priv_view_key_array.len()).step_by(32) {
        let mut priv_key = String::new();
        for j in i..i + 32 {
            priv_key.push_str(&format!("{:02x}", priv_view_key_array[j]));
        }
        priv_view_key.push_str(&priv_key);
    }
    // Finally, return the keys
    vec![priv_spend_key, priv_view_key]
}

/// Derives private spend and view keys from given hex seed
pub fn derive_priv_keys(hex_seed: String) -> Vec<String> {
    match hex_seed.len() {
        32 => derive_mymonero_priv_keys(hex_seed),
        64 => derive_original_priv_keys(hex_seed),
        _ => panic!("Invalid hex seed"),
    }
}

/// Derives private view key from private spend key
pub fn derive_priv_vk_from_priv_sk(private_spend_key: String) -> String {
    // Turn private spend key into bytes and pass through Keccak256 function
    let priv_spend_key_bytes = hex::decode(private_spend_key.clone()).unwrap();
    let priv_view_key_bytes = Keccak256::digest(priv_spend_key_bytes);
    let mut priv_view_key_array = [0u8; 32];
    priv_view_key_array.copy_from_slice(&priv_view_key_bytes);
    // Pass bytes through sc_reduce32 function to get private view key
    sc_reduce32(&mut priv_view_key_array as &mut [u8; 32]);
    let mut priv_view_key = String::new();
    for i in (0..priv_view_key_array.len()).step_by(32) {
        let mut priv_key = String::new();
        for j in i..i + 32 {
            priv_key.push_str(&format!("{:02x}", priv_view_key_array[j]));
        }
        priv_view_key.push_str(&priv_key);
    }
    // Finally, return the private view key
    priv_view_key
}

/// Performs scalar multiplication of the Ed25519 base point by a given scalar, yielding a corresponding point on the elliptic curve
fn ge_scalar_mult_base(scalar: &Scalar) -> EdwardsPoint {
    ED25519_BASEPOINT_TABLE.mul(scalar as &Scalar)
}

/// Derives public key from given private key, can be either spend or view key
pub fn derive_pub_key(private_key: String) -> String {
    // Turn private key into bytes
    let private_key_bytes = hex::decode(private_key.clone()).unwrap();
    let mut private_key_array = [0u8; 32];
    private_key_array.copy_from_slice(&private_key_bytes);
    let key_scalar = Scalar::from_bytes_mod_order(private_key_array);
    // Scalar multiplication with the base point
    let result_point = ge_scalar_mult_base(&key_scalar);
    // The result_point now contains the public key
    let public_key_bytes = result_point.compress().to_bytes();
    let mut public_key = String::new();
    for i in (0..public_key_bytes.len()).step_by(32) {
        let mut pub_key = String::new();
        for j in i..i + 32 {
            pub_key.push_str(&format!("{:02x}", public_key_bytes[j]));
        }
        public_key.push_str(&pub_key);
    }
    // Finally, return the public key
    public_key
}

/// Derives public address from given public spend and view keys and network
pub fn derive_address(public_spend_key: String, public_view_key: String, network: u8) -> String {
    let network_byte = match network {
        0 => vec![0x12], // Monero mainnet
        1 => vec![0x35], // Monero testnet
        _ => panic!("Invalid network"),
    };
    let pub_sk_bytes = hex::decode(public_spend_key.clone()).unwrap();
    let pub_vk_bytes = hex::decode(public_view_key.clone()).unwrap();
    let mut data = [&network_byte[..], &pub_sk_bytes[..], &pub_vk_bytes[..]].concat();
    let hash = Keccak256::digest(&data);
    data.append(&mut hash[..4].to_vec());

    base58_monero::encode(&data).unwrap()
}
