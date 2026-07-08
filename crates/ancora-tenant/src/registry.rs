use crate::namespace::Namespace;
use crate::quota::{ResourceQuota, ResourceUsage};
use crate::tenant::{Tenant, TenantStatus};
use std::collections::HashMap;

#[derive(Debug)]
pub enum RegistryError {
    TenantNotFound(String),
    TenantAlreadyExists(String),
    TenantNotActive(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::TenantNotFound(id) => write!(f, "tenant not found: {}", id),
            RegistryError::TenantAlreadyExists(id) => write!(f, "tenant already exists: {}", id),
            RegistryError::TenantNotActive(id) => write!(f, "tenant is not active: {}", id),
        }
    }
}

pub struct TenantRegistry {
    tenants: HashMap<String, Tenant>,
    quotas: HashMap<String, ResourceQuota>,
    usages: HashMap<String, ResourceUsage>,
    namespaces: HashMap<String, Namespace>,
}

impl TenantRegistry {
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
            quotas: HashMap::new(),
            usages: HashMap::new(),
            namespaces: HashMap::new(),
        }
    }

    pub fn register(&mut self, tenant: Tenant, quota: ResourceQuota) -> Result<(), RegistryError> {
        let id = tenant.id.clone();
        if self.tenants.contains_key(&id) {
            return Err(RegistryError::TenantAlreadyExists(id));
        }
        self.namespaces.insert(id.clone(), Namespace::new(&id));
        self.usages.insert(id.clone(), ResourceUsage::new());
        self.quotas.insert(id.clone(), quota);
        self.tenants.insert(id, tenant);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Tenant> {
        self.tenants.get(id)
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Tenant> {
        self.tenants.get_mut(id)
    }

    pub fn quota(&self, id: &str) -> Option<&ResourceQuota> {
        self.quotas.get(id)
    }
    pub fn usage(&self, id: &str) -> Option<&ResourceUsage> {
        self.usages.get(id)
    }
    pub fn usage_mut(&mut self, id: &str) -> Option<&mut ResourceUsage> {
        self.usages.get_mut(id)
    }
    pub fn namespace(&self, id: &str) -> Option<&Namespace> {
        self.namespaces.get(id)
    }
    pub fn namespace_mut(&mut self, id: &str) -> Option<&mut Namespace> {
        self.namespaces.get_mut(id)
    }

    pub fn count(&self) -> usize {
        self.tenants.len()
    }

    pub fn active_tenants(&self) -> Vec<&Tenant> {
        self.tenants.values().filter(|t| t.is_active()).collect()
    }

    pub fn suspended_tenants(&self) -> Vec<&Tenant> {
        self.tenants.values().filter(|t| t.is_suspended()).collect()
    }

    pub fn require_active(&self, id: &str) -> Result<&Tenant, RegistryError> {
        match self.tenants.get(id) {
            None => Err(RegistryError::TenantNotFound(id.to_string())),
            Some(t) if t.status != TenantStatus::Active => {
                Err(RegistryError::TenantNotActive(id.to_string()))
            }
            Some(t) => Ok(t),
        }
    }
}

impl Default for TenantRegistry {
    fn default() -> Self {
        Self::new()
    }
}
