use aes::cipher::generic_array::GenericArray;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, KeyInit};
use anyhow::Context;
use base64::{Engine as _, engine::general_purpose};
use rsa::{Oaep, RsaPrivateKey};
use rsa::{RsaPublicKey, pkcs8::DecodePrivateKey, pkcs8::DecodePublicKey};
use sha2::Sha256;
use std::error::Error;
use std::fs;

pub fn encrypt_file(
    input_file: &str,
    output_file: &str,
    public_key_file: &str,
) -> Result<(), Box<dyn Error>> {
    // First let's load the RSA Public Key
    let public_key_pem = fs::read_to_string(public_key_file)?;
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;

    // Now generate the AES key and Nonce
    let aes_key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&aes_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Encrypt File Content
    let plaintext = fs::read(input_file)?;
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .map_err(|e| anyhow::anyhow!("AES Encryption Failed {:?}", e))?;

    // Encrypt the keyyyyy
    let padding = Oaep::new::<Sha256>();
    let encrypted_key = public_key.encrypt(&mut OsRng, padding, aes_key.as_slice())?;

    let result = format!(
        "{}\n{}\n{}",
        general_purpose::STANDARD.encode(&encrypted_key),
        general_purpose::STANDARD.encode(&nonce),
        general_purpose::STANDARD.encode(&ciphertext)
    );

    fs::write(output_file, result)?;
    println!("Encrypted file written to {}", output_file);
    Ok(())
}

pub fn decrypt_file(
    input_file: &str,
    output_file: &str,
    private_key_file: &str,
) -> Result<(), Box<dyn Error>> {
    let private_key_pem = fs::read_to_string(private_key_file)?;
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)?;

    // Read and split the file
    let content = fs::read_to_string(input_file)?;
    let mut lines = content.lines();

    let encrypt_key_b64 = lines.next().context("Missing encrypted AES Key")?;
    let nonce_b64 = lines.next().context("Missing nonce")?;
    let ciphertext_b64 = lines.collect::<Vec<_>>().join("");

    let encrypted_key = general_purpose::STANDARD.decode(encrypt_key_b64)?;
    let nonce = general_purpose::STANDARD.decode(nonce_b64)?;
    let ciphertext = general_purpose::STANDARD.decode(ciphertext_b64)?;

    // Decrypt AES Key
    let padding = Oaep::new::<Sha256>();
    let aes_key_bytes = private_key.decrypt(padding, &encrypted_key)?;

    // Decrypt the File
    let key = GenericArray::from_slice(&aes_key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&nonce);
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| anyhow::anyhow!("AES Decryption Failed {:?}", e))?;

    fs::write(output_file, plaintext)?;
    println!("Decrypted file written to {}", output_file);
    Ok(())
}
