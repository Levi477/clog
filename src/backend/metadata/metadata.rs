use super::folder::Folder;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use base64::{
    Engine,
    engine::{self, general_purpose},
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, usize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub data: HashMap<String, Folder>,
    created_at: String,
}

impl Metadata {
    pub fn new() -> Self {
        let created_at = Local::now().format("%d/%m/%Y").to_string();
        Metadata {
            data: HashMap::new(),
            created_at,
        }
    }

    pub fn add_latest_folder(&mut self) {
        let date = Local::now().format("%d/%m/%Y").to_string();
        let key = self.data.contains_key(&date);

        match key {
            true => println!("{} folder already exists", date),
            false => {
                let folder = Folder::new();
                self.data.insert(date, folder);
            }
        }
    }

    fn get_serialized_metadata(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn get_base64_encrypted_metadata(
        &self,
        base64_key: &String,
        base64_nonce: &String,
    ) -> String {
        let serialized_data = self.get_serialized_metadata();
        let plaintext = serialized_data.as_bytes();

        // decode key and make key for encryption
        let key_bytes = general_purpose::STANDARD.decode(base64_key).unwrap();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

        // deocde nonce and make nonce for encryption
        let nonce_bytes = general_purpose::STANDARD.decode(base64_nonce).unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // make cipher
        let cipher = Aes256Gcm::new(key);

        // encrypt plaintext
        let ciphertext = cipher.encrypt(nonce, plaintext).unwrap();

        // convert ciphertext
        let base64_ciphertext = general_purpose::STANDARD.encode(&ciphertext);

        base64_ciphertext
    }

    pub fn add_file(&mut self, filename: &str, foldername: &str, offset: usize, length: usize) {
        let folder = self.data.get_mut(foldername);
        match folder {
            Some(f) => {
                f.add_file(filename, offset, length);
            }
            None => println!(
                "metadata/metadata.rs/add_file : {} folder doesn't exist",
                foldername
            ),
        }
    }

    pub fn update_file_nonce(&mut self, filename: &String, foldername: &String) {
        let folder = self.data.get_mut(foldername);
        match folder {
            Some(f) => {
                f.update_nonce(filename);
            }
            None => println!(
                "metadata/metadata.rs/update_nonce : {} folder doesn't exist",
                foldername
            ),
        }
    }

    pub fn update_all_file_offset(&mut self, delta_offset: usize) {
        for (_, folder) in self.data.iter_mut() {
            for (_, file) in folder.data.iter_mut() {
                file.update_offset(delta_offset);
            }
        }
    }

    pub fn parse_base64_encrypted_metadata(
        base64_encrypted_metadata: &String,
        base64_key: &String,
        base64_nonce: &String,
    ) -> Self {
        // extract key from base64
        let key_bytes = engine::general_purpose::STANDARD
            .decode(base64_key)
            .unwrap();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

        // extract nonce from base64
        let nonce_bytes = engine::general_purpose::STANDARD
            .decode(base64_nonce)
            .unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // extract encrypted_metadata from base64
        let ciphertext_bytes = engine::general_purpose::STANDARD
            .decode(base64_encrypted_metadata)
            .unwrap();

        // make a cipher from key
        let cipher = Aes256Gcm::new(key);

        // decrypt encrypted_metadata
        let metadata_serialized_bytes = cipher.decrypt(&nonce, ciphertext_bytes.as_ref()).unwrap();

        // Convert decrypted bytes to String
        let metadata_serialized = String::from_utf8(metadata_serialized_bytes).unwrap();

        // Deserialize metadata to struct
        serde_json::from_str(&metadata_serialized).unwrap()
    }
}
