use super::utils::{open_file_read, open_file_read_write};
use crate::backend::{
    header::utils::{parse_header_from_file, update_metadata_offset_and_length_in_file},
    metadata::metadata::Metadata,
};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use base64::{Engine, engine::general_purpose};
use chrono::NaiveTime;
use std::{
    io::{Read, Seek, SeekFrom, Write},
    isize,
    path::PathBuf,
};

/// 1. Update nonce of file in metadata
/// 2. Copy all content of files below given file
/// 3. Update all offset of below file and metadata in metadata and header section
/// 4. Write new edited content and paste old copied content
/// 5. Write new updated metadata
pub fn edit_file_with_content(
    metadata: &mut Metadata,
    password: &String,
    foldername: &str,
    filename: &str,
    new_content: &str,
    clogfile_path: &PathBuf,
) {
    // 1. Update nonce of file in metadata

    let folder = metadata.folders.get_mut(foldername).unwrap();

    // update new nonce in metadata
    folder.files.get_mut(filename).unwrap().update_nonce();

    // get new parameters from file — clone/copy to avoid borrow conflicts
    let (base64_key, base64_nonce, offset, old_length) = {
        let (k, n, o, l) = folder.files.get(filename).unwrap().get_file_parameters();
        (k.clone(), n.clone(), o, l) // clone key & nonce so borrow ends here
    };

    let base64_encrypted_content =
        encrypt_and_encode_content_to_base64(new_content, &base64_key, &base64_nonce);

    // update new length of file
    folder
        .files
        .get_mut(filename)
        .unwrap()
        .update_length(base64_encrypted_content.len());

    let delta_offset: isize = base64_encrypted_content.len() as isize - old_length as isize;

    // 2. Copy all content of files given below file

    let mut file = open_file_read_write(clogfile_path);

    file.seek(SeekFrom::Start((offset + old_length).try_into().unwrap()))
        .unwrap();
    let mut below_file_content_bytes = Vec::new();
    file.read_to_end(&mut below_file_content_bytes).unwrap();

    // 3. Update all offset of below file and metadata

    // update metadata offset in header section
    update_metadata_offset_and_length_in_file(clogfile_path, delta_offset, 0);

    // update all offset of below file

    // import basefile time for comparison
    let basefile_time = {
        let base_created_at = &folder.files.get(filename).unwrap().created_at;
        NaiveTime::parse_from_str(base_created_at, "%I:%M:%S %p").unwrap()
    };

    // check all the files in current folder and update all the offset
    for (_, file_) in folder.files.iter_mut() {
        let time = NaiveTime::parse_from_str(&file_.created_at, "%I:%M:%S %p").unwrap();
        if time > basefile_time {
            file_.update_offset(delta_offset);
        }
    }

    // 4. Write new edited content and paste old copied content

    // write new content
    file.seek(SeekFrom::Start(offset.try_into().unwrap()))
        .unwrap();
    file.write_all(base64_encrypted_content.as_bytes()).unwrap();

    // write old content
    file.seek(SeekFrom::Start(
        (offset + base64_encrypted_content.len()) as u64,
    ))
    .unwrap();
    file.write_all(&below_file_content_bytes).unwrap();

    // 5. Write new updated metadata in file

    metadata.update_metadata_in_file(clogfile_path, password);
}

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
    update_metadata_offset_and_length_in_file(clogfile_path, content_len as isize, 0);

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
