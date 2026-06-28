use crate::tenant::Tenant;
use crate::quota::ResourceQuota;

pub struct TenantBuilder {
    id: String,
    name: String,
    created_tick: u64,
    metadata: Vec<(String, String)>,
    quota: Option<ResourceQuota>,
}

impl TenantBuilder {
    pub fn new(id: impl Into<String>, name: impl Into<String>, created_tick: u64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            created_tick,
            metadata: Vec::new(),
            quota: None,
        }
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }

    pub fn quota(mut self, quota: ResourceQuota) -> Self {
        self.quota = Some(quota);
        self
    }

    pub fn build(self) -> (Tenant, ResourceQuota) {
        let mut tenant = Tenant::new(self.id, self.name, self.created_tick);
        for (k, v) in self.metadata { tenant = tenant.with_metadata(k, v); }
        let quota = self.quota.unwrap_or_else(ResourceQuota::standard);
        (tenant, quota)
    }
}
