use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("enryption failed: {0}")]
    Encrypt(aes_gcm::Error),

    #[error("decryption failed: {0}")]
    Decrypt(aes_gcm::Error),

    #[error("failed encoding bytes to utf8 string: {0}")]
    Utf8Encode(#[from] FromUtf8Error),

    #[error("encrypted data has an invalid length")]
    InvalidCipheredDataLength,

    #[error("failed decoding base64 value: {0}")]
    Base64Decode(#[from] base64::DecodeError),
}
