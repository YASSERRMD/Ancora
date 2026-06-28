use crate::key::{CryptoKey, KeyStatus};
use crate::store::KeyStore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationIssue {
    EmptyKeyMaterial(String),
    DestroyedKeyHasMaterial(String),
    NoActiveKeysForTenant(String),
    ExpiredButActive(String),
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationIssue::EmptyKeyMaterial(id) => write!(f, "Key {} has empty material but is not destroyed", id),
            ValidationIssue::DestroyedKeyHasMaterial(id) => write!(f, "Key {} is DESTROYED but still has material", id),
            ValidationIssue::NoActiveKeysForTenant(tid) => write!(f, "Tenant {} has no active keys", tid),
            ValidationIssue::ExpiredButActive(id) => write!(f, "Key {} is expired but still ACTIVE", id),
        }
    }
}

pub struct KeyValidator;

impl KeyValidator {
    pub fn validate_key(key: &CryptoKey, current_tick: u64) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        if key.status != KeyStatus::Destroyed && key.key_material.is_empty() {
            issues.push(ValidationIssue::EmptyKeyMaterial(key.id.clone()));
        }
        if key.status == KeyStatus::Destroyed && !key.key_material.is_empty() {
            issues.push(ValidationIssue::DestroyedKeyHasMaterial(key.id.clone()));
        }
        if key.status == KeyStatus::Active && key.is_expired(current_tick) {
            issues.push(ValidationIssue::ExpiredButActive(key.id.clone()));
        }
        issues
    }

    pub fn validate_tenant(store: &KeyStore, tenant_id: &str, current_tick: u64) -> Vec<ValidationIssue> {
        let active = store.list_tenant_active(tenant_id);
        let mut issues = Vec::new();
        if active.is_empty() {
            issues.push(ValidationIssue::NoActiveKeysForTenant(tenant_id.to_string()));
        }
        for key in store.list_tenant_active(tenant_id) {
            issues.extend(Self::validate_key(key, current_tick));
        }
        issues
    }

    pub fn is_valid_key(key: &CryptoKey, current_tick: u64) -> bool {
        Self::validate_key(key, current_tick).is_empty()
    }
}
