//! Algorithms module
//!
//! Purpose:
//! - Houses pure cryptographic logic with no UI dependencies.
//! - Each file should expose small, testable functions.
//!
//! Conventions:
//! - Keep I/O (reading/writing files) minimal and explicit.
//! - Prefer returning `Result<T, anyhow::Error>` for fallible functions.
//! - Name functions with clear verbs: `encrypt_*`, `decrypt_*`, `generate_*`.
//!
//! Contents:
//! - `aes_only.rs`: AES-256-GCM encrypt/decrypt helpers (symmetric only)
//! - `rsa_hybrid.rs`: RSA+AES hybrid file encryption
//! - `classical_ciphers.rs`: Caesar, Vigenère, Playfair, Hill
//! - `gen_key_pair.rs`: RSA key generation
//! - `math.rs`: number theory helpers (e.g., extended Euclid)
//! - `sign.rs`: signatures (WIP)
pub mod aes_only;
pub mod classical_ciphers;
pub mod gen_key_pair;
pub mod hash;
pub mod math;
pub mod rsa_hybrid;
pub mod sign;
