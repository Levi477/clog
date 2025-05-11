use aes_gcm::aead::{OsRng, rand_core::RngCore};
use base64::{Engine, engine};

pub fn generate_base64_salt() -> String {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let base64_salt = engine::general_purpose::STANDARD.encode(salt);
    base64_salt
}

pub fn generate_base64_nonce() -> String {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let base64_nonce = engine::general_purpose::STANDARD.encode(nonce);
    base64_nonce
}
