use crate::{IsolationChecker, IsolationResult, TenantRegistry};
#[test]
fn different_tenant_is_cross_tenant_violation() {
    let registry = TenantRegistry::new();
    let result = IsolationChecker::check(&registry, "t1", "t2");
    assert!(matches!(
        result,
        IsolationResult::CrossTenantViolation { .. }
    ));
}
#[test]
fn require_same_tenant_fails_cross_tenant() {
    let registry = TenantRegistry::new();
    let result = IsolationChecker::require_same_tenant(&registry, "t1", "t2");
    assert!(result.is_err());
}
