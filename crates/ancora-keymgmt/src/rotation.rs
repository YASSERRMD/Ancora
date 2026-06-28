use crate::key::{CryptoKey, KeyStatus};
use crate::store::{KeyStore, KeyStoreError};

pub struct RotationPolicy {
    pub max_versions: usize,
    pub rotate_after_ticks: Option<u64>,
}

impl RotationPolicy {
    pub fn new(max_versions: usize) -> Self {
        Self { max_versions, rotate_after_ticks: None }
    }

    pub fn with_rotation_interval(mut self, ticks: u64) -> Self {
        self.rotate_after_ticks = Some(ticks);
        self
    }

    pub fn should_rotate(&self, key: &CryptoKey, current_tick: u64) -> bool {
        if let Some(interval) = self.rotate_after_ticks {
            return current_tick >= key.created_tick + interval;
        }
        false
    }
}

pub fn rotate_key(
    store: &mut KeyStore,
    tenant_id: &str,
    id: &str,
    new_material: impl Into<String>,
    tick: u64,
) -> Result<u32, KeyStoreError> {
    let (new_version, algorithm, purpose) = {
        let latest = store.get_latest_mut(tenant_id, id)?;
        let v = latest.version + 1;
        let algo = latest.algorithm.clone();
        let purpose = latest.purpose.clone();
        latest.status = KeyStatus::Inactive;
        (v, algo, purpose)
    };
    let mut new_key = CryptoKey::new(id, tenant_id, algorithm, purpose, tick, new_material);
    new_key.version = new_version;
    store.add_version(tenant_id, id, new_key)?;
    Ok(new_version)
}
