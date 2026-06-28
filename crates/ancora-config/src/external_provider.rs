use crate::{error::ConfigError, secret_provider::SecretProvider};

/// Interface for external secret managers (Vault / OpenBao / AWS SSM etc.).
/// Production implementations would make authenticated HTTP calls.
/// This adapter uses a synchronous fetch callback for offline testing.
pub struct ExternalSecretProvider<F>
where
    F: Fn(&str) -> Result<String, String> + Send + Sync,
{
    provider_name: String,
    fetch: F,
}

impl<F> ExternalSecretProvider<F>
where
    F: Fn(&str) -> Result<String, String> + Send + Sync,
{
    pub fn new(provider_name: impl Into<String>, fetch: F) -> Self {
        Self { provider_name: provider_name.into(), fetch }
    }
}

impl<F> SecretProvider for ExternalSecretProvider<F>
where
    F: Fn(&str) -> Result<String, String> + Send + Sync,
{
    fn resolve(&self, key: &str) -> Result<String, ConfigError> {
        (self.fetch)(key).map_err(|e| ConfigError::SecretUnresolvable {
            key: format!("{}:{} — {}", self.provider_name, key, e),
        })
    }

    fn on_rotation(&mut self, _key: &str) {
        // External managers handle rotation server-side; nothing to cache-bust here.
    }
}
