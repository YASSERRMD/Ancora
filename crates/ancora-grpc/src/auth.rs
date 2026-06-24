use tonic::{Request, Status};

/// Configuration for token-based authentication.
pub struct AuthConfig {
    pub token: String,
}

impl AuthConfig {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
}
