use aes_gcm::aead::{OsRng, rand_core::RngCore};
use base64::{Engine as _, engine::general_purpose};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct File {
    created_at: String,
    offset: usize,
    length: usize,
    key: String,
    nonce: String,
}

impl File {
    pub fn new(offset: usize, length: usize) -> Self {
        // fetch current time
        let created_at = Local::now().format("%I:%M %p").to_string();

        // fill key with random bytes
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let key = general_purpose::STANDARD.encode(&key);

        // fill nonce with random bytes
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let nonce = general_purpose::STANDARD.encode(&nonce);

        File {
            created_at,
            offset,
            length,
            key,
            nonce,
        }
    }

    pub fn update_nonce(&mut self) {
        let mut tmp_nonce = [0u8; 12];
        OsRng.fill_bytes(&mut tmp_nonce);
        let tmp_nonce = general_purpose::STANDARD.encode(&tmp_nonce);
        self.nonce = tmp_nonce;
    }

    pub fn update_offset(&mut self, delta_offset: usize) {
        self.offset += delta_offset;
    }

    pub fn update_length(&mut self, length: usize) {
        self.length = length;
    }
}
