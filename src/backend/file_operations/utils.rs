use super::{super::metadata, content::add_file_with_content};
use crate::backend::{
    header::utils::{HEADER_LENGTH, init},
    user::utils::generate_keys::{generate_base64_nonce, generate_base64_salt},
};
use chrono::Local;
use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    path::PathBuf,
};

fn write_from_offset(file: &mut File, content: &str, offset: usize) {
    file.seek(SeekFrom::Start(offset.try_into().unwrap()))
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn open_file_read_write(path: &PathBuf) -> File {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    file
}

pub fn open_file_read(path: &PathBuf) -> File {
    let file = OpenOptions::new().read(true).open(path).unwrap();
    file
}

pub fn make_new_clogfile(password: &String, clogfile_path: &PathBuf) {
    let mut metadata = metadata::init::init();
    let base64_salt = generate_base64_salt();
    let base64_nonce = generate_base64_nonce();

    // used 312 as temp metadata length
    let (header_line1, header_line2) =
        init("0.0.1", 312, HEADER_LENGTH, &base64_salt, &base64_nonce);

    let mut file = open_file_read_write(clogfile_path);

    // write header section

    file.write_all(header_line1.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
    file.write_all(header_line2.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    // foldername
    let foldername = Local::now().format("%d/%m/%Y").to_string();

    // Welcome file parameteres
    let content =
        "This is the first log.\nEnjoy clog.\nWrite log everyday.\nMake note of everything.";

    // write file content in file
    add_file_with_content(
        &mut metadata,
        password,
        &foldername,
        "Welcome",
        content,
        clogfile_path,
    );
}
