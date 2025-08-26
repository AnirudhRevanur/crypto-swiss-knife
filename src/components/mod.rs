//! UI components for cipher interactions
//!
//! Purpose:
//! - TUI widgets implementing `CipherComponent` for user interaction.
//! - Keep state (inputs, mode, status) here and delegate logic to `algorithms`.
//!
//! Conventions:
//! - Implement `CipherComponent` for each cipher UI.
//! - Handle keyboard events locally; no global state mutations.
//! - Keep rendering self-contained and pure.
//!
//! Contents:
//! - `cipher_component.rs`: trait defining the component interface
//! - `caesar.rs`, `vigenere.rs`, `playfair.rs`: classical cipher UIs
//! - `aes.rs`: AES file encrypt/decrypt UI
pub mod aes;
pub mod caesar;
pub mod cipher_component;
pub mod hash;
pub mod playfair;
pub mod vigenere;
