//! Top-level tabs aggregating cipher components
//!
//! Purpose:
//! - Provide high-level navigation between groups of ciphers.
//! - Each tab manages selection/editing modes for a set of components.
//!
//! Conventions:
//! - Tabs do not implement cryptographic logic; they orchestrate components.
//! - Expose a simple API: `render(...)`, `handle_event(...)`, `is_editing()`.
//! - Keep per-tab state local to the tab struct.
//!
//! Contents:
//! - `classical.rs`: Caesar, Vigenère, Playfair group
//! - `symmetric.rs`: AES group (and future symmetric ciphers)
pub mod classical;
pub mod misc;
pub mod symmetric;
