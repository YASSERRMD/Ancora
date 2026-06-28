use thiserror::Error;

#[derive(Debug, Error)]
pub enum TenantError {
    #[error("tenant not found: {0}")]
    NotFound(String),
    #[error("access denied: tenant {requester} cannot access resource owned by {owner}")]
    CrossTenantAccess { requester: String, owner: String },
    #[error("tenant is suspended: {0}")]
    Suspended(String),
    #[error("tenant is deleted: {0}")]
    Deleted(String),
    #[error("provider {provider} not in allowlist for tenant {tenant}")]
    ProviderNotAllowed { tenant: String, provider: String },
    #[error("residency region mismatch: tenant requires {required}, resource is in {actual}")]
    ResidencyMismatch { required: String, actual: String },
}
