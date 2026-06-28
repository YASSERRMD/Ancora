use crate::component::{Component, ComponentKind, License};
use crate::policy::{PolicyDecision, SupplyChainPolicy};
fn make_component(supplier: &str) -> Component {
    Component::new("c1", "lib", "1.0", ComponentKind::Library, License::Mit, supplier, "d1")
}
#[test]
fn allowed_supplier_permits_component() {
    let policy = SupplyChainPolicy::new("t1").allow_supplier("trusted-vendor");
    let decision = policy.check_component(&make_component("trusted-vendor"), true, true);
    assert_eq!(decision, PolicyDecision::Allow);
}
#[test]
fn unlisted_supplier_is_denied() {
    let policy = SupplyChainPolicy::new("t1").allow_supplier("trusted-vendor");
    let decision = policy.check_component(&make_component("unknown-vendor"), true, true);
    assert!(matches!(decision, PolicyDecision::Deny(_)));
}
#[test]
fn multiple_allowed_suppliers_work() {
    let policy = SupplyChainPolicy::new("t1")
        .allow_supplier("vendor-a")
        .allow_supplier("vendor-b");
    assert!(policy.check_component(&make_component("vendor-a"), true, true).is_allow());
    assert!(policy.check_component(&make_component("vendor-b"), true, true).is_allow());
    assert!(!policy.check_component(&make_component("vendor-c"), true, true).is_allow());
}
