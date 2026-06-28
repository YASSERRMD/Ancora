#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SynthError {
    InvalidSchema(String),
    SandboxViolation(String),
    NotApproved(String),
    PermissionDenied(String),
    NotFound(String),
}

impl std::fmt::Display for SynthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynthError::InvalidSchema(m) => write!(f, "invalid schema: {m}"),
            SynthError::SandboxViolation(m) => write!(f, "sandbox violation: {m}"),
            SynthError::NotApproved(m) => write!(f, "not approved: {m}"),
            SynthError::PermissionDenied(m) => write!(f, "permission denied: {m}"),
            SynthError::NotFound(m) => write!(f, "not found: {m}"),
        }
    }
}
