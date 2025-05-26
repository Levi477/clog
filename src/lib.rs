use std::path::PathBuf;
mod backend;
use backend::main;
use chrono::Local;

/// Daily check and update
fn daily_check_and_update_metadata(clogfile_path: &str, password: &str) {
    let path = PathBuf::from(clogfile_path);
    main::daily_check_and_update_metadata(&path, password);
}

/// Adds a folder
fn add_folder(clogfile_path: &str, password: &str) {
    let path = PathBuf::from(clogfile_path);
    main::add_folder(&path, password).unwrap();
}

/// Adds a new user
pub fn add_new_user(password: &str, clogfile_path: &str) {
    let path = PathBuf::from(clogfile_path);
    main::add_new_user(&path, password);
}

/// Edits a file
pub fn update_file_content(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
    new_file_content: &str,
) {
    let path = PathBuf::from(clogfile_path);
    main::edit_file(password, &path, filename, foldername, new_file_content);
}

/// Decrypt file
pub fn get_file_content(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
) -> String {
    let path = PathBuf::from(clogfile_path);
    main::get_file_content(&path, filename, foldername, password)
}

/// Adds a file
pub fn add_file(password: &str, clogfile_path: &str, filename: &str, file_content: &str) {
    let foldername = Local::now().format("%d/%m/%Y").to_string();
    let path = PathBuf::from(clogfile_path);
    daily_check_and_update_metadata(clogfile_path, password);
    main::add_file(password, &path, filename, &foldername, file_content).unwrap();
}

/// Get Metadata in json_serialized
pub fn get_json_metadata(password: &str, clogfile_path: &str) -> String {
    let path = PathBuf::from(clogfile_path);
    main::get_clean_metadata(password, &path)
}
