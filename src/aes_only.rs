// This function and file is for encrypting using AES ONLY. No RSA included. This is symmetric key
// cryptography

use aes::cipher::generic_array::GenericArray;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, KeyInit};
use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use std::fs;

pub fn encrypt_with_aes(input_file: &str, output_file: &str, key_file: &str) -> Result<()> {
    let aes_key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&aes_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let plaintext = fs::read(input_file)?;
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .map_err(|e| anyhow::anyhow!("AES Encryption Failed {:?}", e))?;

    let encrypted_data = format!(
        "{}\n{}",
        general_purpose::STANDARD.encode(&nonce),
        general_purpose::STANDARD.encode(&ciphertext)
    );

    fs::write(output_file, &encrypted_data)?;

    let encoded_key = general_purpose::STANDARD.encode(&aes_key);
    fs::write(key_file, encoded_key)?;

    println!(
        "AES Encryption done. Encrypted file written to {} and key written to {}",
        output_file, key_file
    );
    Ok(())
}

pub fn decrypt_with_aes(input_file: &str, output_file: &str, key_file: &str) -> Result<()> {
    let content = fs::read_to_string(input_file)?;
    let mut lines = content.lines();

    let nonce_b64 = lines.next().context("Missing nonce")?;
    let ciphertext_b64 = lines.collect::<Vec<_>>().join("");

    let nonce_bytes = general_purpose::STANDARD.decode(nonce_b64)?;
    let ciphertext = general_purpose::STANDARD.decode(ciphertext_b64)?;

    let key_b64 = fs::read_to_string(key_file)?;
    let key_bytes = general_purpose::STANDARD.decode(key_b64.trim())?;

    let key = GenericArray::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| anyhow::anyhow!("AES Decryption Failed: {e}"))?;

    fs::write(output_file, plaintext)?;
    println!("Decrypted file written to {}", output_file);
    Ok(())
}
