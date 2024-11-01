use crate::encryption;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decrypt(#[from] encryption::errors::Error),

    #[error("failed decoding yaml: {0}")]
    Decode(#[from] serde_yaml::Error),

    #[error("no profile exists with name {0}")]
    NoProfileWithName(String),

    #[error("encryption key must be provided")]
    NoEncryptionKey,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
