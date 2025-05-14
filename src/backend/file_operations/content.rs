use super::utils::{open_file_read, open_file_read_write};
use crate::backend::{
    header::utils::{parse_header_from_file, update_metadata_offset_and_length_in_file},
    metadata::metadata::Metadata,
};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use base64::{Engine, engine::general_purpose};
use std::{
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

/// 1. Updates local metadata to include new file
/// 2. Adds file content in the clogfile
/// 3. Updates header section
/// 4. Updates metadata section
pub fn add_file_with_content(
    metadata: &mut Metadata,
    password: &String,
    foldername: &str,
    filename: &str,
    content: &str,
    clogfile_path: &PathBuf,
) {
    // get metadata offset from header
    let (_, _, _, metadata_offset, _) = parse_header_from_file(clogfile_path);

    // update local metadata to include new file
    metadata.add_file(filename, foldername, metadata_offset, 0);

    // get file parameters to encrypt the content
    let (base64_key, base64_nonce, _, _) = metadata
        .folders
        .get(foldername)
        .unwrap()
        .files
        .get(filename)
        .unwrap()
        .get_file_parameters();

    // encrypt and encode content to base64
    let base64_encrypted_content =
        encrypt_and_encode_content_to_base64(content, base64_key, base64_nonce);
    let content_len = base64_encrypted_content.len();

    println!(
        "encrypted file content : {}\nfile length : {}\n",
        base64_encrypted_content, content_len
    );

    // update length of file in local metadata
    metadata
        .folders
        .get_mut(foldername)
        .unwrap()
        .files
        .get_mut(filename)
        .unwrap()
        .update_length(content_len);

    // update header section to update metadata_offset
    update_metadata_offset_and_length_in_file(clogfile_path, content_len, 0);

    // write file content in the clogfile
    let mut file = open_file_read_write(clogfile_path);
    file.seek(SeekFrom::Start(metadata_offset.try_into().unwrap()))
        .unwrap();
    file.write_all(base64_encrypted_content.as_bytes()).unwrap();

    // update metadata in file
    metadata.update_metadata_in_file(clogfile_path, password);
}

pub fn decrypt_content_from_file(
    metadata: &Metadata,
    foldername: &str,
    filename: &str,
    clogfile_path: &PathBuf,
) -> String {
    // open file in read only mode
    let mut file = open_file_read(clogfile_path);

    // get parameters of file to be decrypted
    let (base64_key, base64_nonce, offset, length) = metadata
        .folders
        .get(foldername)
        .unwrap()
        .files
        .get(filename)
        .unwrap()
        .get_file_parameters();

    // get base64_encrypted_content from file
    file.seek(SeekFrom::Start(offset.try_into().unwrap()))
        .unwrap();

    // make a container to store bytes from file
    let mut base64_encrypted_content = vec![0u8; length];

    // read bytes from file and convert bytes to string
    file.read_exact(&mut base64_encrypted_content).unwrap();
    let base64_encrypted_content = String::from_utf8(base64_encrypted_content).unwrap();

    // decrypt base64_encrypted_content and return string
    parse_base64_encrypted_data(&base64_encrypted_content, &base64_key, &base64_nonce)
}

pub fn parse_base64_encrypted_data(
    base64_encrypted_data: &String,
    base64_key: &String,
    base64_nonce: &String,
) -> String {
    println!(
        "Parameters passed to parse_base64_encrypted_data :\nbase64_encrypted_content : {},\nbase64_key : {},\nbase64_nonce : {}\n",
        base64_encrypted_data, base64_key, base64_nonce
    );

    // extract key from base64
    let key_bytes = general_purpose::STANDARD.decode(base64_key).unwrap();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // extract nonce from base64
    let nonce_bytes = general_purpose::STANDARD.decode(base64_nonce).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // extract encrypted_metadata from base64
    let ciphertext_bytes = general_purpose::STANDARD
        .decode(base64_encrypted_data)
        .unwrap();

    // make a cipher from key
    let cipher = Aes256Gcm::new(key);

    // decrypt encrypted_metadata
    let data_bytes = cipher.decrypt(&nonce, ciphertext_bytes.as_ref()).unwrap();

    // Convert decrypted bytes to String
    String::from_utf8(data_bytes).unwrap()
}

fn encrypt_and_encode_content_to_base64(
    content: &str,
    base64_key: &str,
    base64_nonce: &str,
) -> String {
    // make plaintext from content by converting it to bytes
    let plaintext = content.as_bytes();

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
