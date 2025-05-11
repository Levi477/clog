use std::{
    fs::File,
    io::{BufRead, BufReader},
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

pub fn parse_header_from_file(file: &File) -> (String, String, usize, usize, String) {
    let mut header_line1 = String::new();
    let mut header_line2 = String::new();

    let buf_reader = BufReader::new(file);
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

pub fn update_metadata_offset_and_length(file: &File, delta_offset: usize, length: usize) {
    todo!()
}

// pub fn update_nonce_in_file

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::parse_header_from_file;
    #[test]
    fn test_parse_header() {
        let file = File::open("deep.clog").unwrap();
        let (a, b, c, d, e) = parse_header_from_file(&file);
        println!(
            "salt : {},nonce : {},metadata_length : {},metadata_offset : {},version_id : {}",
            a, b, c, d, e
        );
    }
}
