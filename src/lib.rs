/*
 * This file is part of Monume's library libmonero
 *
 * Copyright (c) 2023-2024, Monume (monume.xyz)
 * All Rights Reserved
 * The code is distributed under MIT license, see LICENSE file for details.
 * Generated by Monume
 *
 */

#![allow(clippy::module_inception)]
#![allow(dead_code)]

//! Powerful, batteries-included Monero library. It is mainly function-oriented, but some structs are also included.
//! 
//! You can get started by adding the "libmonero" crate in your project: \
//! `cargo add libmonero`
//! 
//! Below list is sorted alphabetically.
//! 
//! ## Structs, Functions And All Usable Items
//! 
//! - Blocks
//!     - Nodes
//!         - [`DaemonNode`](blocks/struct.DaemonNode.html)
//!             - [`cake_wallet_default()`](blocks/struct.DaemonNode.html#method.cake_wallet_default)
//!             - [`new(url: String, port: u16, tls: bool)`](blocks/struct.DaemonNode.html#method.new)
//!             - [`stack_wallet_default()`](blocks/struct.DaemonNode.html#method.stack_wallet_default)
//!     - RPCs
//!         - [`get_height(node: DaemonNode) -> u64`](blocks/fn.get_height.html)
//!         - [`get_block_from_height(node: DaemonNode, height: u64) -> Block`](blocks/fn.get_block_from_height.html)
//!         - [`get_transaction_from_hash(node: DaemonNode, hash: &str) -> RawTx`](blocks/fn.get_transaction_from_hash.html)
//! - Crypt
//!     - [`cryptonight`](crypt/cryptonight/index.html)
//!         - [`cn_slow_hash(input: &[u8]) -> String`](crypt/cryptonight/fn.cn_slow_hash.html) - EXPERIMENTAL!
//! - Keys
//!     - [`derive_address(public_spend_key: String, public_view_key: String, network: i8) -> String`](keys/fn.derive_address.html)
//!     - [`derive_hex_seed(mnemonic_seed: Vec<String>) -> String`](keys/fn.derive_hex_seed.html)
//!     - [`derive_priv_keys(hex_seed: String) -> Vec<String>`](keys/fn.derive_priv_keys.html)
//!     - [`derive_priv_vk_from_priv_sk(private_spend_key: String) -> String`](keys/fn.derive_priv_vk_from_priv_sk.html)
//!     - [`derive_pub_key(private_key: String) -> String`](keys/fn.derive_pub_key.html)
//!     - [`generate_seed(language: &str, seed_type: &str) -> Vec<String>`](keys/fn.generate_seed.html)
//! - Utils
//!     - [`is_valid_addr(address: &str) -> bool`](utils/fn.is_valid_addr.html)

pub(crate) use mnemonics::original::wordsets;

pub(crate) mod mnemonics {
    pub mod original {
        pub mod wordsets;
        pub mod languages {
            pub mod chinese_simplified;
            pub mod dutch;
            pub mod english;
            pub mod esperanto;
            pub mod french;
            pub mod german;
            pub mod italian;
            pub mod japanese;
            pub mod lojban;
            pub mod portuguese;
            pub mod russian;
            pub mod spanish;
        }
    }
}

/// Cryptographic functions
pub mod crypt;
/// Block manipulation functions
pub mod blocks;
/// Key manipulation functions
pub mod keys;
/// Utility functions like address validation
pub mod utils;

// Will be added in the future
// pub mod wallet;
// pub use wallet::*;