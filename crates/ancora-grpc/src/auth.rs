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

/// Tonic interceptor that validates a Bearer token in the Authorization header.
#[derive(Clone)]
pub struct AuthInterceptor {
    expected: String,
}

impl AuthInterceptor {
    pub fn new(token: impl Into<String>) -> Self {
        Self { expected: token.into() }
    }
}
