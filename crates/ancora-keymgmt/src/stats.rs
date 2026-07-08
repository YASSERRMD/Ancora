use crate::key::KeyStatus;
use crate::store::KeyStore;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KeyStats {
    pub tenant_id: String,
    pub total_active: usize,
    pub total_inactive: usize,
    pub total_compromised: usize,
    pub total_destroyed: usize,
    pub by_algorithm: HashMap<String, usize>,
}

impl KeyStats {
    pub fn for_tenant(store: &KeyStore, tenant_id: &str) -> Self {
        let active = store.list_tenant_active(tenant_id);
        let total_active = active.len();
        let mut by_algorithm: HashMap<String, usize> = HashMap::new();
        for k in &active {
            *by_algorithm.entry(format!("{}", k.algorithm)).or_insert(0) += 1;
        }
        Self {
            tenant_id: tenant_id.to_string(),
            total_active,
            total_inactive: 0,
            total_compromised: 0,
            total_destroyed: 0,
            by_algorithm,
        }
    }
}

pub struct KeyStatusSummary {
    pub key_id: String,
    pub version_count: usize,
    pub current_status: KeyStatus,
}
