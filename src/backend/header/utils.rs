use crate::backend::{
    file_operations::utils::{open_file_read, open_file_read_write},
    user::utils::generate_keys::generate_base64_nonce,
};
use std::{
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
};

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

pub fn parse_header_from_file(clogfile_path: &PathBuf) -> (String, String, usize, usize, String) {
    let file = open_file_read(clogfile_path);

    let mut header_line1 = String::new();
    let mut header_line2 = String::new();
    let buf_reader = BufReader::new(&file);
    let mut line_reader = buf_reader.lines();

    header_line1 = line_reader.next().unwrap().unwrap();
    header_line2 = line_reader.next().unwrap().unwrap();

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

pub fn update_metadata_offset_and_length_in_file(
    clogfile_path: &PathBuf,
    delta_offset: usize,
    length: usize,
) {
    let mut file = open_file_read_write(clogfile_path);

    let (base64_salt, base64_nonce, mut metadata_length, mut metadata_offset, _) =
        parse_header_from_file(clogfile_path);
    metadata_length = length;
    metadata_offset += delta_offset;
    let header_line2 = format!(
        "{}.{}.{:08}.{:08}\n",
        base64_salt, base64_nonce, metadata_length, metadata_offset
    );
    println!("{}", header_line2);
    file.seek(SeekFrom::Start(12)).unwrap();
    file.write_all(header_line2.as_bytes()).unwrap();
}

pub fn update_nonce_in_file(clogfile_path: &PathBuf) {
    let mut file = open_file_read_write(clogfile_path);

    let (base64_salt, mut base64_nonce, metadata_length, metadata_offset, _) =
        parse_header_from_file(clogfile_path);
    base64_nonce = generate_base64_nonce();
    let header_line2 = format!(
        "{}.{}.{:08}.{:08}\n",
        base64_salt, base64_nonce, metadata_length, metadata_offset
    );

    file.seek(SeekFrom::Start(12)).unwrap();
    file.write_all(header_line2.as_bytes()).unwrap();
}
#[cfg(test)]
mod test {}
