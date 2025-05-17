use super::{
    file_operations::{
        content::{add_file_with_content, decrypt_content_from_file, edit_file_with_content},
        utils::make_new_clogfile,
    },
    metadata::metadata::Metadata,
};
use chrono::Local;
use serde_json::{Value, json};
use std::path::PathBuf;

pub fn get_file_content(
    clogfile_path: &PathBuf,
    filename: &str,
    foldername: &str,
    password: &str,
) -> String {
    let metadata = Metadata::extract_metadata_from_file(clogfile_path, password);
    decrypt_content_from_file(&metadata, foldername, filename, clogfile_path)
}

pub fn add_new_user(clogfile_path: &PathBuf, password: &str) {
    make_new_clogfile(&password.to_string(), clogfile_path);
}

pub fn get_clean_metadata(password: &str, clogfile_path: &PathBuf) -> String {
    // Extract full metadata
    let metadata: Metadata = Metadata::extract_metadata_from_file(clogfile_path, password);

    // Prepare JSON map for folders
    let mut folders_json = serde_json::Map::new();

    for (folder_name, folder) in metadata.folders {
        let mut folder_json = serde_json::Map::new();

        for (file_name, file) in folder.files {
            // Only include created_at per file
            folder_json.insert(file_name, json!({ "created_at": file.created_at }));
        }

        folders_json.insert(folder_name, Value::Object(folder_json));
    }

    // Build root JSON with folders + top-level created_at
    let mut root_json = serde_json::Map::new();
    root_json.insert("folders".to_string(), Value::Object(folders_json));
    root_json.insert("created_at".to_string(), json!(metadata.created_at));

    serde_json::to_string_pretty(&Value::Object(root_json)).unwrap()
}

pub fn add_file(
    password: &str,
    clogfile_path: &PathBuf,
    filename: &str,
    foldername: &str,
    file_content: &str,
) -> Result<(), String> {
    let mut metadata = Metadata::extract_metadata_from_file(clogfile_path, password);

    // check if filename exists in folder
    let folder = metadata.folders.get_mut(foldername).unwrap();

    // if file does exists return error message
    if folder.files.get(filename).is_some() {
        return Err("filename already exists!".to_string());
    }

    // extract metadata from file
    let mut metadata = Metadata::extract_metadata_from_file(clogfile_path, password);

    // if file doesn't exist than add file with content
    add_file_with_content(
        &mut metadata,
        &password.to_string(),
        foldername,
        filename,
        file_content,
        clogfile_path,
    );

    Ok(())
}

pub fn add_folder(clogfile_path: &PathBuf, password: &str) -> Result<(), String> {
    // extract metadata from file
    let mut metadata = Metadata::extract_metadata_from_file(clogfile_path, password);

    let foldername = Local::now().format("%d/%m/%Y").to_string();

    // check if folder exists in metadata
    if metadata.folders.get(&foldername).is_some() {
        return Err(String::from("folder already exists"));
    }

    // if folder doesn't exists than make new one using current date
    metadata.add_latest_folder();

    Ok(())
}

pub fn edit_file(
    password: &str,
    clogfile_path: &PathBuf,
    filename: &str,
    foldername: &str,
    new_file_content: &str,
) {
    let mut metadata = Metadata::extract_metadata_from_file(clogfile_path, password);

    edit_file_with_content(
        &mut metadata,
        &password.to_string(),
        foldername,
        filename,
        new_file_content,
        clogfile_path,
    );
}

pub fn daily_check_and_update_metadata(clogfile_path: &PathBuf, password: &str) {
    let mut metadata = Metadata::extract_metadata_from_file(clogfile_path, password);
    let mut current_folder_exists = false;
    let current_date = Local::now().format("%d/%m/%Y").to_string();

    for (folder_date, folder) in metadata.folders.iter_mut() {
        if *folder_date != current_date {
            folder.make_read_only();
        } else {
            current_folder_exists = true;
        }

        if !current_folder_exists {
            add_folder(clogfile_path, password).unwrap();
        }
    }
}
