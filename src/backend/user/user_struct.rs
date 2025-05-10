use std::path::{Path, PathBuf};

use aes_gcm::aead::{OsRng, rand_core::RngCore};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;

use crate::backend::header;

pub struct User {
    username: String,
    total_logs: u16,
    total_folders: u16,
    salt: [u8; 16],
    nonce: [u8; 12],
}

impl User {
    pub fn new(username: String) -> Self {
        let mut salt = [0u8; 16];
        let mut nonce = [0u8; 12];

        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce);

        User {
            username,
            total_logs: 0,
            total_folders: 1,
            salt,
            nonce,
        }
    }

    // pub fn init_header(&self, filepath: PathBuf) -> Result<(), String> {
    //     header::init::init(version_id);
    // Ok(())
    // }
}
