use super::super::metadata;
use crate::backend::{
    header::utils::{HEADER_LENGTH, init},
    user::utils::{
        derive_key::derive_key_base64,
        generate_keys::{generate_base64_nonce, generate_base64_salt},
    },
};
use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
};

fn write_from_offset(file: &mut File, content: &str, offset: usize) {
    file.seek(SeekFrom::Start(offset.try_into().unwrap()))
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn make_new_clogfile(username: &str, password: &str) -> File {
    let metadata = metadata::init::init();
    let base64_salt = generate_base64_salt();
    let base64_nonce = generate_base64_nonce();

    let base64_key = derive_key_base64(password, &base64_salt);

    let encrypted_metadata = metadata.get_base64_encrypted_metadata(&base64_key, &base64_nonce);

    let metadata_length = encrypted_metadata.len();

    let (header_line1, header_line2) = init(
        "0.0.1",
        metadata_length,
        HEADER_LENGTH,
        &base64_salt,
        &base64_nonce,
    );

    let mut file = File::create(format!("{}.clog", username)).unwrap();

    // write header section

    file.write_all(header_line1.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
    file.write_all(header_line2.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    // write metadata section

    println!("encrypted metadata : {}", encrypted_metadata);

    write_from_offset(&mut file, &encrypted_metadata, HEADER_LENGTH);

    file
}

// fn add_file(metadata: &mut Metadata, filename: &str, clog_file: &mut File) {
//     let foldername = Local::now().format("%d/%m/%Y").to_string();
//     let mut offset: usize = 0;
//     let files_hashmap = metadata.data.get_mut(&foldername).unwrap();
//     for (_, prev_file) in files_hashmap.data.iter_mut() {
//         prev_file.off
//     }
//
//     metadata.add_file(filename, &foldername, offset, 0);
// }

#[cfg(test)]
mod test {

    // #[test]
    // pub fn test_make_new_file() {
    //     todo!()
    // }
}
