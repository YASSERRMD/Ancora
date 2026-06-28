use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenantState { Active, Suspended }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantEntry {
    pub tenant_id: String,
    pub state: TenantState,
}

#[derive(Default)]
pub struct TenantOps {
    tenants: HashMap<String, TenantEntry>,
}

impl TenantOps {
    pub fn create(&mut self, tenant_id: impl Into<String>) {
        let id: String = tenant_id.into();
        self.tenants.insert(id.clone(), TenantEntry { tenant_id: id, state: TenantState::Active });
    }

    pub fn suspend(&mut self, tenant_id: &str) -> bool {
        if let Some(t) = self.tenants.get_mut(tenant_id) {
            t.state = TenantState::Suspended;
            return true;
        }
        false
    }

    pub fn list(&self) -> Vec<&TenantEntry> {
        let mut v: Vec<&TenantEntry> = self.tenants.values().collect();
        v.sort_by_key(|t| &t.tenant_id);
        v
    }

    pub fn get(&self, tenant_id: &str) -> Option<&TenantEntry> {
        self.tenants.get(tenant_id)
    }
}
