use aes_gcm::aead::OsRng;
use rsa::{
    RsaPrivateKey, RsaPublicKey,
    pkcs8::{EncodePrivateKey, EncodePublicKey},
};
use std::{error::Error, fs::write};

pub fn generate_key_pair(
    private_key_file: &str,
    public_key_file: &str,
) -> Result<(), Box<dyn Error>> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let public_key = RsaPublicKey::from(&private_key);

    let private_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)?
        .to_string();
    let public_pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)?;

    write(private_key_file, private_pem.as_bytes())?;
    write(public_key_file, public_pem)?;

    println!("Key pair generated!");

    Ok(())
}
