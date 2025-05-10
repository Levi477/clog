use super::folder::Folder;
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

    // fn get_metadata_length(&self)

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
