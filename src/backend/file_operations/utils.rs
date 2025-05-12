use super::super::metadata;
use crate::backend::{
    header::utils::{HEADER_LENGTH, init, update_metadata_offset_and_length_in_file},
    user::utils::generate_keys::{generate_base64_nonce, generate_base64_salt},
};
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

// // this function currently doesn't work hence using above function every time
// pub fn open_file_read(path: &PathBuf) -> File {
//     println!("Trying to open file at path: {:?}", path);
//
//     let file = OpenOptions::new()
//         .create(true)
//         .read(true)
//         .open(path)
//         .unwrap();
//     file
// }

pub fn make_new_clogfile(password: &String, clogfile_path: &PathBuf) {
    let metadata = metadata::init::init();
    let base64_salt = generate_base64_salt();
    let base64_nonce = generate_base64_nonce();

    // used 312 as temp metadata length
    let (header_line1, header_line2) =
        init("0.0.1", 312, HEADER_LENGTH, &base64_salt, &base64_nonce);

    let mut file = open_file_read_write(clogfile_path);
    println!("file created succesfully");

    // write header section

    file.write_all(header_line1.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
    file.write_all(header_line2.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    println!("header section written in file succesfully");

    let encrypted_metadata = metadata.to_base64_encrypted_metadata(&password, clogfile_path);

    let metadata_length = encrypted_metadata.len();

    // write metadata section

    write_from_offset(&mut file, &encrypted_metadata, HEADER_LENGTH);

    // update tmp metadata_length to real length
    update_metadata_offset_and_length_in_file(clogfile_path, 0, metadata_length);
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use crate::backend::{file_operations::utils::make_new_clogfile, metadata::metadata::Metadata};
    use std::env;
    #[test]
    fn test_updated_metadata() {
        let mut clogfile_path = env::current_dir().unwrap();
        clogfile_path.push("deep.clog");
        let password = String::from("deep0904");
        make_new_clogfile(&password, &clogfile_path);
        let mut metadata = Metadata::extract_metadata_from_file(&clogfile_path, &password);

        println!(
            "extracted metadata from file : {}",
            serde_json::to_string(&metadata).unwrap()
        );
        metadata.add_file(
            "testfile.txt",
            &Local::now().format("%d/%m/%Y").to_string(),
            10,
            90,
        );
        metadata.update_metadata_in_file(&clogfile_path, &password);
        metadata = Metadata::extract_metadata_from_file(&clogfile_path, &password);

        println!(
            "extracted metadata from file : {}",
            serde_json::to_string(&metadata).unwrap()
        );
    }
}
