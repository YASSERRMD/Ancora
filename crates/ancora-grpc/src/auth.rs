use tonic::{Request, Status};

/// Configuration for token-based authentication.
pub struct AuthConfig {
    pub token: String,
}
