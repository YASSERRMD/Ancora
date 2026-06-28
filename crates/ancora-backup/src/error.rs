use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("serialization failed: {0}")]
    Serialization(String),
    #[error("deserialization failed: {0}")]
    Deserialization(String),
    #[error("checksum mismatch: archive is corrupt or tampered")]
    ChecksumMismatch,
    #[error("restore error: {0}")]
    Restore(String),
}
