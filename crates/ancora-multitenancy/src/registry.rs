use crate::error::TenantError;
use crate::tenant::{Tenant, TenantConfig, TenantId, TenantState};
use std::collections::HashMap;

/// In-memory store of all tenants. In production this would back onto the journal store.
#[derive(Default)]
pub struct TenantRegistry {
    tenants: HashMap<TenantId, Tenant>,
}

impl TenantRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&mut self, name: impl Into<String>, config: TenantConfig) -> TenantId {
        let tenant = Tenant::new(name, config);
        let id = tenant.id.clone();
        self.tenants.insert(id.clone(), tenant);
        id
    }

    pub fn get(&self, id: &TenantId) -> Option<&Tenant> {
        self.tenants.get(id)
    }

    pub fn suspend(&mut self, id: &TenantId) -> Result<(), TenantError> {
        let t = self
            .tenants
            .get_mut(id)
            .ok_or(TenantError::NotFound(id.0.clone()))?;
        t.state = TenantState::Suspended;
        Ok(())
    }

    pub fn delete(&mut self, id: &TenantId) -> Result<(), TenantError> {
        let t = self
            .tenants
            .get_mut(id)
            .ok_or(TenantError::NotFound(id.0.clone()))?;
        t.state = TenantState::Deleted;
        Ok(())
    }

    pub fn is_active(&self, id: &TenantId) -> bool {
        self.tenants.get(id).map(|t| t.is_active()).unwrap_or(false)
    }

    pub fn list(&self) -> Vec<&Tenant> {
        self.tenants.values().collect()
    }
}
