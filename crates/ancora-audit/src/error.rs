use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum AuditError {
    ChecksumMismatch { id: u64 },
    LogAtCapacity { max: usize },
    EntryNotFound { id: u64 },
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditError::ChecksumMismatch { id } => {
                write!(f, "checksum mismatch for entry id={}", id)
            }
            AuditError::LogAtCapacity { max } => write!(f, "audit log at capacity (max={})", max),
            AuditError::EntryNotFound { id } => write!(f, "audit entry id={} not found", id),
        }
    }
}
