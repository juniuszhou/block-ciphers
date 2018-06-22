//! This crate is a wrapper around different implementations of AES block ciphers.
//!
//! Currently it uses:
//! - [`aes-soft`](https://docs.rs/aes-soft) hardware independent bit-sliced
//! implementation
//! - [`aesni`](https://docs.rs/aesni) implementation using
//! [AES-NI](https://en.wikipedia.org/wiki/AES_instruction_set) instruction set.
//! Used for x86-64 and x86 target architectures with enabled "aes" target feature.
//!
//! Crate switches between implementations automatically at compile time.
//! (i.e. it does not use run-time feature detection) If `enforce_soft` crate
//! feature is enabled software implementation will be used regardless of target
//! architecture and enabled target features.
//!
//! # Usage example
//! ```
//! # use aes::block_cipher_trait::generic_array::GenericArray;
//! use aes::{Aes128, BlockCipher};
//!
//! let key = GenericArray::from_slice(&[0u8; 16]);
//! let mut block = GenericArray::clone_from_slice(&[0u8; 16]);
//! let mut block8 = GenericArray::clone_from_slice(&[block; 8]);
//! // Initialize cipher
//! let cipher = Aes128::new(&key);
//!
//! let block_copy = block.clone();
//! // Encrypt block in-place
//! cipher.encrypt_block(&mut block);
//! // And decrypt it back
//! cipher.decrypt_block(&mut block);
//! assert_eq!(block, block_copy);
//!
//! // We can encrypt 8 blocks simultaneously using
//! // instruction-level parallelism
//! let block8_copy = block8.clone();
//! cipher.encrypt_blocks(&mut block8);
//! cipher.decrypt_blocks(&mut block8);
//! assert_eq!(block8, block8_copy);
//! ```
//!
//! For implementations of block cipher modes of operation see
//! [`block-modes`](https://docs.rs/block-modes) crate.
#![no_std]
pub extern crate block_cipher_trait;
extern crate aes_soft;
#[cfg(all(
    not(feature = "enforce_soft"),
    target_feature = "aes",
    any(target_arch = "x86_64", target_arch = "x86"),
))]
extern crate aesni;

pub use block_cipher_trait::BlockCipher;

#[cfg(
    all(
        not(feature = "enforce_soft"),
        target_feature="aes",
        any(target_arch = "x86_64", target_arch = "x86"),
    )
)]
pub use aesni::{Aes128, Aes192, Aes256};

#[cfg(any(
    feature = "enforce_soft",
    // aesni cfg
    not(all(
        target_feature="aes",
        any(target_arch = "x86_64", target_arch = "x86"),
    ))
))]
pub use aes_soft::{Aes128, Aes192, Aes256};