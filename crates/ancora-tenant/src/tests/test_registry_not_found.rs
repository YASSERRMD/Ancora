use crate::TenantRegistry;
#[test]
fn get_missing_tenant_returns_none() {
    let registry = TenantRegistry::new();
    assert!(registry.get("ghost").is_none());
}
#[test]
fn require_active_missing_tenant_returns_error() {
    let registry = TenantRegistry::new();
    assert!(registry.require_active("ghost").is_err());
}
