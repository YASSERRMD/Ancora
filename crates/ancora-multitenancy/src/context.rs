use crate::tenant::TenantId;

/// Attached to every operation to enforce tenant-scoped access.
#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant_id: TenantId,
}

impl TenantContext {
    pub fn new(tenant_id: TenantId) -> Self {
        Self { tenant_id }
    }

    /// Prefix a storage key (journal table, vector collection, etc.) with the tenant id
    /// so that tenant-A data is never reachable by tenant-B queries.
    pub fn scope_key(&self, key: &str) -> String {
        format!("tenant:{}:{}", self.tenant_id.as_str(), key)
    }

    /// Returns `true` if `owner_id` matches this context's tenant id.
    pub fn owns(&self, owner_id: &TenantId) -> bool {
        &self.tenant_id == owner_id
    }
}
