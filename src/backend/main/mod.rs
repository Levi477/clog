use chrono::Local;

use super::{
    file_operations::content::{add_file_with_content, edit_file_with_content},
    metadata::{folder, metadata::Metadata},
};
use std::path::PathBuf;

pub fn add_file(
    metadata: &mut Metadata,
    password: &str,
    clogfile_path: &PathBuf,
    filename: &str,
    foldername: &str,
    file_content: &str,
) -> Result<(), String> {
    // check if filename exists in folder
    let folder = metadata.folders.get_mut(foldername).unwrap();

    // if file does exists return error message
    if folder.files.get(filename).is_some() {
        return Err("filename already exists!".to_string());
    }

    // if file doesn't exist than add file with content
    add_file_with_content(
        metadata,
        &password.to_string(),
        foldername,
        filename,
        file_content,
        clogfile_path,
    );

    Ok(())
}

pub fn add_folder(metadata: &mut Metadata) -> Result<(), String> {
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
    metadata: &mut Metadata,
    password: &str,
    clogfile_path: &PathBuf,
    filename: &str,
    foldername: &str,
    new_file_content: &str,
) {
    edit_file_with_content(
        metadata,
        &password.to_string(),
        foldername,
        filename,
        new_file_content,
        clogfile_path,
    );
}

pub fn daily_check_and_update_metadata(metadata: &mut Metadata) {
    let current_date = Local::now().format("%d/%m/%Y").to_string();

    for (folder_date, folder) in metadata.folders.iter_mut() {
        if *folder_date != current_date {
            folder.make_read_only();
        }
    }
}
