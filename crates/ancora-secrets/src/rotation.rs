use crate::store::SecretStore;
use crate::error::SecretError;

pub struct RotationPolicy {
    pub max_versions: usize,
}

impl RotationPolicy {
    pub fn new(max_versions: usize) -> Self { Self { max_versions } }

    pub fn default_policy() -> Self { Self { max_versions: 10 } }

    pub fn rotate(
        &self,
        store: &mut SecretStore,
        tenant_id: &str,
        path: &str,
        new_value: impl Into<String>,
        tick: u64,
    ) -> Result<u32, SecretError> {
        let new_version = store.write_version(tenant_id, path, new_value, tick)?;
        let secret = store.read_mut(tenant_id, path)?;
        if secret.versions.len() > self.max_versions {
            let excess = secret.versions.len() - self.max_versions;
            secret.versions.drain(0..excess);
        }
        Ok(new_version)
    }

    pub fn versions_retained(&self, store: &SecretStore, tenant_id: &str, path: &str) -> Result<usize, SecretError> {
        let secret = store.read(tenant_id, path)?;
        Ok(secret.version_count())
    }
}
