use crate::error::SecretError;
use crate::store::SecretStore;

pub struct ExpiryChecker;

impl ExpiryChecker {
    pub fn is_expired(
        store: &SecretStore,
        tenant_id: &str,
        path: &str,
        current_tick: u64,
    ) -> Result<bool, SecretError> {
        let secret = store.read(tenant_id, path)?;
        Ok(secret.is_expired(current_tick))
    }

    pub fn expired_paths(store: &SecretStore, tenant_id: &str, current_tick: u64) -> Vec<String> {
        store
            .list_tenant(tenant_id)
            .into_iter()
            .filter(|s| s.is_expired(current_tick))
            .map(|s| s.path.clone())
            .collect()
    }

    pub fn active_paths(store: &SecretStore, tenant_id: &str, current_tick: u64) -> Vec<String> {
        store
            .list_tenant(tenant_id)
            .into_iter()
            .filter(|s| !s.is_expired(current_tick))
            .map(|s| s.path.clone())
            .collect()
    }
}
