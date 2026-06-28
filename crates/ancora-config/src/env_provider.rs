use std::collections::HashMap;
use crate::{error::ConfigError, secret_provider::SecretProvider};

/// Resolves secrets from environment variables (or an in-memory override for testing).
pub struct EnvSecretProvider {
    /// Overrides injected at construction (useful in tests without real env vars).
    overrides: HashMap<String, String>,
    /// Invalidated keys (simulating rotation).
    invalidated: std::collections::HashSet<String>,
}

impl EnvSecretProvider {
    pub fn new() -> Self {
        Self { overrides: HashMap::new(), invalidated: std::collections::HashSet::new() }
    }

    pub fn with_override(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.overrides.insert(key.into(), value.into());
        self
    }
}

impl Default for EnvSecretProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretProvider for EnvSecretProvider {
    fn resolve(&self, key: &str) -> Result<String, ConfigError> {
        if self.invalidated.contains(key) {
            return Err(ConfigError::SecretUnresolvable { key: key.into() });
        }
        if let Some(v) = self.overrides.get(key) {
            return Ok(v.clone());
        }
        std::env::var(key).map_err(|_| ConfigError::SecretUnresolvable { key: key.into() })
    }

    fn on_rotation(&mut self, key: &str) {
        // After rotation, invalidate cached value; next resolve picks up fresh value.
        self.invalidated.insert(key.into());
        self.overrides.remove(key);
    }
}
