use crate::error::ConfigError;

/// Trait for secret providers. Implementors resolve a named key to its value.
/// Secrets are never stored in config structs or logs.
pub trait SecretProvider: Send + Sync {
    fn resolve(&self, key: &str) -> Result<String, ConfigError>;

    /// Called when a secret has been rotated; implementations invalidate caches.
    fn on_rotation(&mut self, key: &str);
}

/// A secret reference embedded in config pointing to an external key.
#[derive(Clone, Debug)]
pub struct SecretRef {
    /// The provider name (e.g. "env", "file", "vault").
    pub provider: String,
    /// The key within that provider.
    pub key: String,
}

impl SecretRef {
    pub fn new(provider: impl Into<String>, key: impl Into<String>) -> Self {
        Self { provider: provider.into(), key: key.into() }
    }
}
