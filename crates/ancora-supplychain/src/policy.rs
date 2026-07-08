use crate::component::{Component, License};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
}

impl PolicyDecision {
    pub fn is_allow(&self) -> bool {
        matches!(self, PolicyDecision::Allow)
    }
}

#[derive(Debug, Clone)]
pub struct SupplyChainPolicy {
    pub tenant_id: String,
    pub deny_licenses: HashSet<String>,
    pub require_signature: bool,
    pub require_provenance: bool,
    pub allowed_suppliers: Option<HashSet<String>>,
}

impl SupplyChainPolicy {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            deny_licenses: HashSet::new(),
            require_signature: false,
            require_provenance: false,
            allowed_suppliers: None,
        }
    }

    pub fn deny_license(mut self, license: License) -> Self {
        self.deny_licenses.insert(format!("{}", license));
        self
    }

    pub fn require_signature(mut self) -> Self {
        self.require_signature = true;
        self
    }

    pub fn require_provenance(mut self) -> Self {
        self.require_provenance = true;
        self
    }

    pub fn allow_supplier(mut self, supplier: impl Into<String>) -> Self {
        self.allowed_suppliers
            .get_or_insert_with(HashSet::new)
            .insert(supplier.into());
        self
    }

    pub fn check_component(
        &self,
        component: &Component,
        has_sig: bool,
        has_prov: bool,
    ) -> PolicyDecision {
        let lic_str = format!("{}", component.license);
        if self.deny_licenses.contains(&lic_str) {
            return PolicyDecision::Deny(format!("license {} is denied", lic_str));
        }
        if self.require_signature && !has_sig {
            return PolicyDecision::Deny(format!(
                "component {} lacks required signature",
                component.id
            ));
        }
        if self.require_provenance && !has_prov {
            return PolicyDecision::Deny(format!(
                "component {} lacks required provenance",
                component.id
            ));
        }
        if let Some(allowed) = &self.allowed_suppliers {
            if !allowed.contains(component.supplier.as_str()) {
                return PolicyDecision::Deny(format!(
                    "supplier {} not in allowlist",
                    component.supplier
                ));
            }
        }
        PolicyDecision::Allow
    }
}
