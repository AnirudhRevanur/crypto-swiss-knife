# Algorithms

Pure cryptographic logic. No UI. Prefer small, testable functions that accept/return data, not TUI state.

## Add a new algorithm
1. Create a new file, e.g. `my_cipher.rs`.
2. Implement clear entrypoints, e.g. `encrypt_*`, `decrypt_*`, `generate_*`.
3. Return `Result<T, anyhow::Error>` for fallible functions.
4. Register your module in `mod.rs` with `pub mod my_cipher;`.

## Template
Copy as `src/algorithms/my_cipher.rs` and adapt.

```rust
use anyhow::Result;

pub fn my_cipher_encrypt(plaintext: &str, key: &str) -> Result<String> {
    // TODO: implement
    Ok(format!("encrypted:{}:{}", key, plaintext))
}

pub fn my_cipher_decrypt(ciphertext: &str, key: &str) -> Result<String> {
    // TODO: implement
    Ok(format!("decrypted:{}:{}", key, ciphertext))
}
```

## File I/O policy
- Keep file reading/writing minimal and explicit if needed.
- Prefer pure APIs that work on `&[u8]`/`&str`. Let UI handle file paths.
