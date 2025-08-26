use digest::Digest;
use md5::compute;
use sha2::{Sha256, Sha512};

fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    bytes
        .as_ref()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn md5_hex(data: &[u8]) -> String {
    let digest = compute(data);
    to_hex(digest.0)
}

pub fn sha1_hex(data: &[u8]) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(data);
    to_hex(hasher.finalize())
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    to_hex(hasher.finalize())
}

pub fn sha512_hex(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    to_hex(hasher.finalize())
}

pub fn hash_str_md5(s: &str) -> String {
    md5_hex(s.as_bytes())
}
pub fn hash_str_sha1(s: &str) -> String {
    sha1_hex(s.as_bytes())
}
pub fn hash_str_sha256(s: &str) -> String {
    sha256_hex(s.as_bytes())
}
pub fn hash_str_sha512(s: &str) -> String {
    sha512_hex(s.as_bytes())
}
