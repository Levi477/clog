use super::metadata::Metadata;

/// Gives predefined structure to initialize metadata
/// {
///     created_at : "09/05/2025 9:25AM",
///     data : [
///         "09/05/2025" : [
///                 "welcome.txt" : {
///                     created_at : "9:30 AM",
///                     offset : 72,
///                     length : 0,
///                     key : "randomely_generated_32bytes_key",
///                     nonce : "randomely_generated_12byte_key",
///                     read_only : true
///                 },
///         ],                    
///     ]
/// }
///
pub fn init() -> Metadata {
    let mut metadata = Metadata::new();
    metadata.add_latest_folder();
    metadata
}
