use super::folder::Folder;
use aes_gcm::{Aes256Gcm, AesGcm, Key, KeyInit, Nonce, aead::Aead, aes::cipher};
use base64::{Engine, engine::general_purpose};
use chrono::Local;
// use linked_hash_map::LinkedHashMap as HashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    data: HashMap<String, Folder>,
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

    pub fn get_encrypted_metadata(&self, base64_key: &str, base64_nonce: &str) -> String {
        let serialized_data = self.get_serialized_metadata();
        let plaintext = serialized_data.as_bytes();

        // decode key and make key for encryption
        let key_bytes = general_purpose::STANDARD.decode(base64_key).unwrap();
        let key = Key::from_slice(&key_bytes);

        // deocde nonce and make nonce for encryption
        let nonce_bytes = general_purpose::STANDARD.decode(base64_nonce).unwrap();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // make cipher
        let cipher = Aes256Gcm::new(key);

        // encrypt plaintext
        let ciphertext = cipher.encrypt(nonce, plaintext);

        // convert ciphertext
        let base64_ciphertext = general_purpose::STANDARD.encode(&ciphertext);

        base64_ciphertext
    }

    pub fn add_file(
        &mut self,
        filename: &String,
        foldername: &String,
        offset: usize,
        length: usize,
    ) {
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

    pub fn update_nonce(&mut self, filename: &String, foldername: &String) {
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

    // fn update_offset_and_length(
    //     &mut self,
    //     filename: &String,
    //     foldername: &String,
    //     delta_offset: usize,
    //     length: usize,
    // ) {
    //     let folder = self.data.get_mut(foldername);
    //     match folder {
    //         Some(f) => f.update_offset_and_length(filename, delta_offset, length),
    //         None => println!(
    //             "metadata/metadata.rs/update_offset_and_length : {} folder doesn't exist",
    //             foldername
    //         ),
    //     }
    // }

    pub fn update_all_offset(&mut self, delta_offset: usize) {
        for (_, folder) in self.data.iter_mut() {
            for (_, file) in folder.data.iter_mut() {
                file.update_offset(delta_offset);
            }
        }
    }
}
