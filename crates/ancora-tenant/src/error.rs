use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TenantError {
    NotFound(String),
    AlreadyExists(String),
    NotActive(String),
    QuotaExceeded {
        resource: String,
        used: u64,
        max: u64,
    },
    CrossTenantViolation {
        subject: String,
        resource_tenant: String,
    },
}

impl fmt::Display for TenantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TenantError::NotFound(id) => write!(f, "tenant not found: {}", id),
            TenantError::AlreadyExists(id) => write!(f, "tenant already exists: {}", id),
            TenantError::NotActive(id) => write!(f, "tenant not active: {}", id),
            TenantError::QuotaExceeded {
                resource,
                used,
                max,
            } => write!(f, "{} quota exceeded: used={} max={}", resource, used, max),
            TenantError::CrossTenantViolation {
                subject,
                resource_tenant,
            } => write!(
                f,
                "cross-tenant violation: {} accessing {}",
                subject, resource_tenant
            ),
        }
    }
}
