use crate::{IsolationChecker, IsolationResult, TenantRegistry};
#[test]
fn same_tenant_is_isolated() {
    let registry = TenantRegistry::new();
    let result = IsolationChecker::check(&registry, "t1", "t1");
    assert_eq!(result, IsolationResult::Isolated);
}
