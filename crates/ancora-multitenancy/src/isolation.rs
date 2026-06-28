use crate::error::TenantError;
use crate::registry::TenantRegistry;
use crate::tenant::{TenantId, TenantState};

/// Enforcement layer called by control plane, workers, and storage adapters.
pub struct TenantIsolation<'a> {
    registry: &'a TenantRegistry,
}

impl<'a> TenantIsolation<'a> {
    pub fn new(registry: &'a TenantRegistry) -> Self {
        Self { registry }
    }

    /// Ensure the tenant exists and is active before accepting a run.
    pub fn assert_active(&self, tenant_id: &TenantId) -> Result<(), TenantError> {
        let t = self.registry.get(tenant_id).ok_or_else(|| TenantError::NotFound(tenant_id.0.clone()))?;
        match t.state {
            TenantState::Active => Ok(()),
            TenantState::Suspended => Err(TenantError::Suspended(tenant_id.0.clone())),
            TenantState::Deleted => Err(TenantError::Deleted(tenant_id.0.clone())),
        }
    }

    /// Deny access when requester tenant tries to read/write another tenant's resource.
    pub fn assert_owns(&self, requester: &TenantId, owner: &TenantId) -> Result<(), TenantError> {
        if requester == owner {
            Ok(())
        } else {
            Err(TenantError::CrossTenantAccess {
                requester: requester.0.clone(),
                owner: owner.0.clone(),
            })
        }
    }

    /// Validate that the requested provider is in this tenant's allowlist.
    pub fn assert_provider_allowed(&self, tenant_id: &TenantId, provider: &str) -> Result<(), TenantError> {
        let t = self.registry.get(tenant_id).ok_or_else(|| TenantError::NotFound(tenant_id.0.clone()))?;
        if t.config.provider_allowlist.is_empty() || t.config.provider_allowlist.iter().any(|p| p == provider) {
            Ok(())
        } else {
            Err(TenantError::ProviderNotAllowed {
                tenant: tenant_id.0.clone(),
                provider: provider.to_owned(),
            })
        }
    }

    /// Validate that the resource's region satisfies the tenant's residency policy.
    pub fn assert_residency(&self, tenant_id: &TenantId, resource_region: &str) -> Result<(), TenantError> {
        let t = self.registry.get(tenant_id).ok_or_else(|| TenantError::NotFound(tenant_id.0.clone()))?;
        match &t.config.residency_region {
            None => Ok(()),
            Some(req) if req == resource_region => Ok(()),
            Some(req) => Err(TenantError::ResidencyMismatch {
                required: req.clone(),
                actual: resource_region.to_owned(),
            }),
        }
    }
}
