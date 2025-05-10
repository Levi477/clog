use aes_gcm::aead::{OsRng, rand_core::RngCore};
use base64::{Engine, engine};
use std::path::PathBuf;

pub const HEADER_LENGTH: usize = 72;

/// initiate header string for the first time when user is created
///
/// # Returns :
///
/// (header_line1,header_line2)
///
pub fn init(
    version_id: &str,
    metadata_length: usize,
    metadata_offset: usize,
    base64_salt: &str,
    base64_nonce: &str,
) -> (String, String) {
    let line1 = format!("clog @{}", version_id);
    let line2 = format!(
        "{}.{}.{:08}.{:08}",
        base64_salt, base64_nonce, metadata_length, metadata_offset
    );
    (line1, line2)
}

/// parses header section to give useful information
///
/// # Returns :
///
/// (base64_salt,base64_nonce,metadata_length,metadata_offset,version_id)

pub fn parse_header(
    header_line1: &str,
    header_line2: &str,
) -> (String, String, usize, usize, String) {
    let (_, version_id) = header_line1.trim().split_at(6);
    let array: Vec<&str> = header_line2.trim().split(".").collect();

    (
        array[0].to_string(),
        array[1].to_string(),
        array[2].parse().unwrap(),
        array[3].parse().unwrap(),
        version_id.to_string(),
    )
}

pub fn update_metadata_offset_and_length(filepath: PathBuf, delta_offset: usize, length: usize) {
    todo!()
}

pub fn get_base64_salt() -> String {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let base64_salt = engine::general_purpose::STANDARD.encode(salt);
    base64_salt
}

pub fn get_base64_nonce() -> String {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let base64_nonce = engine::general_purpose::STANDARD.encode(nonce);
    base64_nonce
}
