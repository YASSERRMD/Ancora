use std::collections::HashMap;
use crate::{error::ConfigError, secret_provider::SecretProvider};

/// Resolves secrets from an in-memory map simulating a secrets file store.
/// In production the file is a mounted Kubernetes Secret or Vault agent file.
pub struct FileSecretProvider {
    store: HashMap<String, String>,
    invalidated: std::collections::HashSet<String>,
}

impl FileSecretProvider {
    pub fn new(store: HashMap<String, String>) -> Self {
        Self { store, invalidated: std::collections::HashSet::new() }
    }
}

impl SecretProvider for FileSecretProvider {
    fn resolve(&self, key: &str) -> Result<String, ConfigError> {
        if self.invalidated.contains(key) {
            return Err(ConfigError::SecretUnresolvable { key: key.into() });
        }
        self.store.get(key).cloned().ok_or_else(|| ConfigError::KeyNotFound { key: key.into() })
    }

    fn on_rotation(&mut self, key: &str) {
        self.invalidated.insert(key.into());
    }
}
