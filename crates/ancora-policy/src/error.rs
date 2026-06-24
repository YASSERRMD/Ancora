/// Errors raised when a policy is violated.
#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("residency violation: endpoint '{0}' is not in the allowed list")]
    ResidencyViolation(String),
    #[error("pii detected in field '{0}'")]
    PiiDetected(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("audit required for action '{0}'")]
    AuditRequired(String),
}
