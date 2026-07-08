use crate::store::SecretStore;

#[derive(Debug)]
pub struct SecretSummary {
    pub tenant_id: String,
    pub total: usize,
    pub with_ttl: usize,
    pub version_counts: Vec<usize>,
}

impl SecretSummary {
    pub fn for_tenant(store: &SecretStore, tenant_id: &str) -> Self {
        let secrets = store.list_tenant(tenant_id);
        let total = secrets.len();
        let with_ttl = secrets.iter().filter(|s| s.ttl_ticks.is_some()).count();
        let version_counts = secrets.iter().map(|s| s.version_count()).collect();
        Self {
            tenant_id: tenant_id.to_string(),
            total,
            with_ttl,
            version_counts,
        }
    }

    pub fn total_versions(&self) -> usize {
        self.version_counts.iter().sum()
    }
    pub fn max_versions(&self) -> usize {
        self.version_counts.iter().copied().max().unwrap_or(0)
    }
}
