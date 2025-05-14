use aes_gcm::aead::{OsRng, rand_core::RngCore};
use base64::{Engine as _, engine::general_purpose};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    pub created_at: String,
    pub offset: usize,
    pub length: usize,
    key: String,
    nonce: String,
}

impl File {
    pub fn new(offset: usize, length: usize) -> Self {
        // fetch current time
        let created_at = Local::now().format("%I:%M:%S %p").to_string();

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

    pub fn update_offset(&mut self, delta_offset: isize) {
        self.offset = (self.offset as isize + delta_offset) as usize;
    }

    pub fn update_length(&mut self, length: usize) {
        self.length = length;
    }

    /// gives (base64_key,base64_nonce,offset,length) of file
    pub fn get_file_parameters(&self) -> (&String, &String, usize, usize) {
        (&self.key, &self.nonce, self.offset, self.length)
    }
}
