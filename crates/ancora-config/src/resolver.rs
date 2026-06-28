use std::collections::HashMap;
use crate::{error::ConfigError, secret_provider::SecretProvider};

/// Registry of named providers. Resolves `provider:key` references at use time.
pub struct SecretResolver {
    providers: HashMap<String, Box<dyn SecretProvider>>,
}

impl SecretResolver {
    pub fn new() -> Self {
        Self { providers: HashMap::new() }
    }

    pub fn register(&mut self, name: impl Into<String>, provider: Box<dyn SecretProvider>) {
        self.providers.insert(name.into(), provider);
    }

    /// Resolve a `"provider:key"` reference. Secrets are returned directly to the
    /// caller and never stored in any log or journal buffer.
    pub fn resolve(&self, reference: &str) -> Result<String, ConfigError> {
        let (provider_name, key) = split_ref(reference)?;
        let provider = self.providers.get(provider_name).ok_or_else(|| {
            ConfigError::ProviderNotFound { provider: provider_name.into() }
        })?;
        provider.resolve(key)
    }

    /// Notify the named provider that a key has been rotated.
    pub fn notify_rotation(&mut self, provider_name: &str, key: &str) -> Result<(), ConfigError> {
        let provider = self.providers.get_mut(provider_name).ok_or_else(|| {
            ConfigError::ProviderNotFound { provider: provider_name.into() }
        })?;
        provider.on_rotation(key);
        Ok(())
    }
}

impl Default for SecretResolver {
    fn default() -> Self {
        Self::new()
    }
}

fn split_ref(reference: &str) -> Result<(&str, &str), ConfigError> {
    reference.split_once(':').ok_or_else(|| ConfigError::SecretUnresolvable {
        key: reference.into(),
    })
}
