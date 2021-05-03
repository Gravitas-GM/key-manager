use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum KeyManagerError {
    InvalidProjectDirectory,
    IoError(io::Error),
    InvalidPublicKey,
    PublicKeyMissingComment,
    UnknownError(Box<dyn Error>),
}

impl fmt::Display for KeyManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyManagerError::InvalidProjectDirectory => write!(f, "Could not determine project directory."),
            KeyManagerError::IoError(e) => write!(f, "{}", e),
            KeyManagerError::InvalidPublicKey => write!(f, "Could not parse public key; you probably provided a malformed key"),
            KeyManagerError::PublicKeyMissingComment => write!(f, "You must either provide a key name, or provide a public key with a comment"),
            KeyManagerError::UnknownError(e) => write!(f, "An unknown error occurred: {}", e.as_ref()),
        }
    }
}

impl Error for KeyManagerError {}
