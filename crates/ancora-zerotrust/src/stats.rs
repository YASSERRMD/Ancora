use crate::identity::{Identity, IdentityStatus};
use std::collections::HashMap;

pub struct ZeroTrustStats {
    pub tenant_id: String,
    pub total_identities: usize,
    pub active_identities: usize,
    pub suspended_identities: usize,
    pub by_kind: HashMap<String, usize>,
}

impl ZeroTrustStats {
    pub fn for_tenant(identities: &[&Identity], tenant_id: &str) -> Self {
        let tenant: Vec<&&Identity> = identities
            .iter()
            .filter(|i| i.tenant_id == tenant_id)
            .collect();
        let total_identities = tenant.len();
        let active_identities = tenant.iter().filter(|i| i.is_active()).count();
        let suspended_identities = tenant
            .iter()
            .filter(|i| i.status == IdentityStatus::Suspended)
            .count();
        let mut by_kind = HashMap::new();
        for i in &tenant {
            *by_kind.entry(format!("{}", i.kind)).or_insert(0) += 1;
        }
        Self {
            tenant_id: tenant_id.to_string(),
            total_identities,
            active_identities,
            suspended_identities,
            by_kind,
        }
    }
}
