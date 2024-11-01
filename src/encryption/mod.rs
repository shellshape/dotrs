pub mod errors;

use aes_gcm::{
    aead::{AeadMut, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use errors::{Error, Result};

pub fn encrypt_string(value: &str, key: &str) -> Result<String> {
    let key_bytes = base64_decode(key)?;

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes.as_slice()));

    let ciphered_data = cipher
        .encrypt(&nonce, value.as_bytes())
        .map_err(Error::Encrypt)?;

    let mut combined_data = nonce.to_vec();
    combined_data.extend_from_slice(&ciphered_data);

    Ok(base64_encode(&combined_data))
}

pub fn decrypt_string(encrypted_value: &str, key: &str) -> Result<String> {
    let encrypted_value_bytes = base64_decode(encrypted_value)?;
    let key_bytes = base64_decode(key)?;

    let (nonce_bytes, ciphered_data_bytes) = encrypted_value_bytes
        .split_at_checked(12)
        .ok_or(Error::InvalidCipheredDataLength)?;
    let nonce = Nonce::from_slice(nonce_bytes);

    let mut cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes.as_slice()));

    let plain_bytes = cipher
        .decrypt(nonce, ciphered_data_bytes)
        .map_err(Error::Decrypt)?;

    Ok(String::from_utf8(plain_bytes)?)
}

fn base64_decode(v: &str) -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(v)?)
}

fn base64_encode(v: &[u8]) -> String {
    BASE64_STANDARD.encode(v)
}
