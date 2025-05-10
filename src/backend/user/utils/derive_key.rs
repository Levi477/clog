use base64::{Engine, engine};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha256;

pub fn derive_key_base64(password: &str, base64_salt: &str) -> String {
    let salt = engine::general_purpose::STANDARD
        .decode(base64_salt)
        .unwrap();
    const ITERATIONS: u32 = 100_000;
    const KEY_LENGTH: usize = 32;

    let mut derived_key = [0u8; KEY_LENGTH];

    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), &salt, ITERATIONS, &mut derived_key).unwrap();

    engine::general_purpose::STANDARD.encode(derived_key)
}
