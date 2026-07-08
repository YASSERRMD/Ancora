use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum SecretError {
    NotFound(String),
    AlreadyExists(String),
    InvalidPath(String),
    VersionNotFound { path: String, version: u32 },
    Expired(String),
    AccessDenied(String),
}

impl fmt::Display for SecretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecretError::NotFound(p) => write!(f, "secret not found: {}", p),
            SecretError::AlreadyExists(p) => write!(f, "secret already exists: {}", p),
            SecretError::InvalidPath(msg) => write!(f, "invalid path: {}", msg),
            SecretError::VersionNotFound { path, version } => {
                write!(f, "version {} not found at path {}", version, path)
            }
            SecretError::Expired(p) => write!(f, "secret expired: {}", p),
            SecretError::AccessDenied(p) => write!(f, "access denied to secret: {}", p),
        }
    }
}
