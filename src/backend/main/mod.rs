use chrono::Local;

use super::{
    file_operations::{
        content::{add_file_with_content, edit_file_with_content},
        utils::make_new_clogfile,
    },
    metadata::metadata::Metadata,
};
use std::path::PathBuf;

pub fn add_new_user(clogfile_path: &PathBuf, password: &str) {
    make_new_clogfile(&password.to_string(), clogfile_path);
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

    let current_date = Local::now().format("%d/%m/%Y").to_string();

    for (folder_date, folder) in metadata.folders.iter_mut() {
        if *folder_date != current_date {
            folder.make_read_only();
        }
    }
}
