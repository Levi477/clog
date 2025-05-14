use super::{file::File, folder::Folder};
use crate::backend::{
    file_operations::{
        content::parse_base64_encrypted_data,
        utils::{open_file_read, open_file_read_write},
    },
    header::utils::{
        parse_header_from_file, update_metadata_offset_and_length_in_file, update_nonce_in_file,
    },
    user::utils::derive_key::derive_key_base64,
};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use base64::{Engine, engine::general_purpose};
use chrono::{Local, NaiveTime};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    usize,
};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub folders: HashMap<String, Folder>,
    created_at: String,
}

impl Metadata {
    pub fn new() -> Self {
        let created_at = Local::now().format("%d/%m/%Y").to_string();
        Metadata {
            folders: HashMap::new(),
            created_at,
        }
    }

    pub fn add_latest_folder(&mut self) {
        let date = Local::now().format("%d/%m/%Y").to_string();
        let key = self.folders.contains_key(&date);

        match key {
            true => println!("{} folder already exists", date),
            false => {
                let folder = Folder::new();
                self.folders.insert(date, folder);
            }
        }
    }

    fn get_all_files_under_folder(&self, foldername: &str) -> &HashMap<String, File> {
        &self.folders.get(foldername).unwrap().files
    }

    pub fn update_file_length(&mut self, foldername: &str, filename: &str, length: usize) {
        let filemap = self.folders.get_mut(foldername).unwrap();
        let file = filemap.files.get_mut(filename).unwrap();
        file.update_length(length);
    }

    fn get_all_clone_files_below_given_file(
        &self,
        foldername: &str,
        filename: &str,
    ) -> HashMap<String, File> {
        let filemap = self.get_all_files_under_folder(foldername);
        let basefile_time = &filemap.get(filename).unwrap().created_at;
        let basefile_time = NaiveTime::parse_from_str(&basefile_time, "%I:%M:%S %p").unwrap();

        filemap
            .iter()
            .filter_map(|(name, file)| {
                let file_time = NaiveTime::parse_from_str(&file.created_at, "%I:%M:%S %p").unwrap();
                if file_time > basefile_time {
                    Some((name.clone(), file.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_serialized_metadata(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// gets nonce and salt from file
    /// and convert given metadata to base64_encrypted_metadata using user password

    pub fn to_base64_encrypted_metadata(
        &self,
        password: &String,
        clogfile_path: &PathBuf,
    ) -> String {
        // get nonce,salt and offset fro metadata
        let (base64_salt, base64_nonce, _, _, _) = parse_header_from_file(clogfile_path);

        // derive key using password and salt
        let base64_key = derive_key_base64(password, &base64_salt);

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
        let folder = self.folders.get_mut(foldername);
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
        let folder = self.folders.get_mut(foldername);
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

    fn parse_base64_encrypted_metadata(
        base64_encrypted_metadata: &String,
        base64_key: &String,
        base64_nonce: &String,
    ) -> Self {
        let metadata_serialized =
            parse_base64_encrypted_data(base64_encrypted_metadata, base64_key, base64_nonce);

        // Deserialize metadata to struct
        serde_json::from_str(&metadata_serialized).unwrap()
    }
    pub fn extract_metadata_from_file(clogfile_path: &PathBuf, password: &str) -> Self {
        // get nonce,salt and offset fro metadata
        let (base64_salt, base64_nonce, metadata_length, metadata_offset, _) =
            parse_header_from_file(clogfile_path);

        // derive key using password and salt
        let base64_key = derive_key_base64(password, &base64_salt);

        let mut file = open_file_read(clogfile_path);

        // get the cursur to offset to read metadata
        file.seek(SeekFrom::Start(metadata_offset.try_into().unwrap()))
            .unwrap();

        // make a container to store given bytes from file
        let mut base64_encrypted_metadata_bytes = vec![0u8; metadata_length];

        // read given bytes from file
        file.read(&mut base64_encrypted_metadata_bytes).unwrap();

        // convert given bytes to string
        let base64_encrypted_metadata = String::from_utf8(base64_encrypted_metadata_bytes).unwrap();

        // get metadata struct from given base64_encrypted_metadata
        let metadata = Metadata::parse_base64_encrypted_metadata(
            &base64_encrypted_metadata,
            &base64_key,
            &base64_nonce,
        );

        metadata
    }

    pub fn update_metadata_in_file(&self, clogfile_path: &PathBuf, password: &String) {
        // first update nonce in file
        update_nonce_in_file(clogfile_path);

        // get current base64_encrypted_metadata
        let base64_encrypted_metadata = self.to_base64_encrypted_metadata(password, clogfile_path);

        // get offset from header
        let (_, _, _, metadata_offset, _) = parse_header_from_file(clogfile_path);

        let new_metadata_length = base64_encrypted_metadata.len();

        let mut file = open_file_read_write(clogfile_path);

        // write from that offset and update length in header
        file.seek(SeekFrom::Start(metadata_offset as u64)).unwrap();
        file.write_all(base64_encrypted_metadata.as_bytes())
            .unwrap();

        // update length of metadata in header section
        update_metadata_offset_and_length_in_file(clogfile_path, 0, new_metadata_length);
    }

    // pub fn update_file_length_and_offset()
}
