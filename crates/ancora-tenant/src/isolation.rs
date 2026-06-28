use crate::registry::TenantRegistry;

#[derive(Debug, PartialEq, Eq)]
pub enum IsolationResult {
    Isolated,
    CrossTenantViolation { subject_tenant: String, resource_tenant: String },
}

pub struct IsolationChecker;

impl IsolationChecker {
    pub fn check(
        _registry: &TenantRegistry,
        subject_tenant_id: &str,
        resource_tenant_id: &str,
    ) -> IsolationResult {
        if subject_tenant_id == resource_tenant_id {
            IsolationResult::Isolated
        } else {
            IsolationResult::CrossTenantViolation {
                subject_tenant: subject_tenant_id.to_string(),
                resource_tenant: resource_tenant_id.to_string(),
            }
        }
    }

    pub fn is_isolated(result: &IsolationResult) -> bool {
        *result == IsolationResult::Isolated
    }

    pub fn require_same_tenant(
        _registry: &TenantRegistry,
        subject_tenant_id: &str,
        resource_tenant_id: &str,
    ) -> Result<(), String> {
        match Self::check(_registry, subject_tenant_id, resource_tenant_id) {
            IsolationResult::Isolated => Ok(()),
            IsolationResult::CrossTenantViolation { subject_tenant, resource_tenant } => {
                Err(format!("cross-tenant access denied: {} attempted to access resource in {}", subject_tenant, resource_tenant))
            }
        }
    }
}
