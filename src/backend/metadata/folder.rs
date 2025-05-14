use super::file::File;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Folder {
    pub files: HashMap<String, File>,
    read_only: bool,
}

impl Folder {
    pub fn new() -> Self {
        Folder {
            files: HashMap::new(),
            read_only: false,
        }
    }

    pub fn add_file(&mut self, filename: &str, offset: usize, length: usize) {
        let key = filename.to_string();
        let value = File::new(offset, length);
        self.files.insert(key, value);
    }

    pub fn make_read_only(&mut self) {
        self.read_only = true;
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn update_nonce(&mut self, filename: &str) {
        let file = self.files.get_mut(filename).unwrap();
        file.update_nonce();
    }

    pub fn update_offset(&mut self, filename: &str, delta_offset: isize) {
        let file = self.files.get_mut(filename).unwrap();
        file.update_offset(delta_offset);
    }

    pub fn update_length(&mut self, filename: &str, length: usize) {
        let file = self.files.get_mut(filename).unwrap();
        file.update_length(length);
    }
}
