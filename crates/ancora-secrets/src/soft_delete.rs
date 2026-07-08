use crate::error::SecretError;
use crate::secret::SecretStatus;
use crate::store::SecretStore;

pub fn soft_delete(
    store: &mut SecretStore,
    tenant_id: &str,
    path: &str,
) -> Result<(), SecretError> {
    let secret = store.read_mut(tenant_id, path)?;
    for v in &mut secret.versions {
        if v.status == SecretStatus::Active {
            v.status = SecretStatus::Deleted;
        }
    }
    Ok(())
}

pub fn is_soft_deleted(
    store: &SecretStore,
    tenant_id: &str,
    path: &str,
) -> Result<bool, SecretError> {
    let secret = store.read(tenant_id, path)?;
    let any_active = secret
        .versions
        .iter()
        .any(|v| v.status == SecretStatus::Active);
    Ok(!any_active && !secret.versions.is_empty())
}
