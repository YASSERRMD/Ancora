use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error("validation failed: {field}: {reason}")]
    Validation { field: String, reason: String },

    #[error("secret ref unresolvable: {key}")]
    SecretUnresolvable { key: String },

    #[error("provider not found: {provider}")]
    ProviderNotFound { provider: String },

    #[error("key not found: {key}")]
    KeyNotFound { key: String },

    #[error("rotation failed: {reason}")]
    RotationFailed { reason: String },

    #[error("serialization error: {0}")]
    Serialization(String),
}
