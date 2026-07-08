use crate::component::{Component, ComponentKind, License};
use crate::policy::{PolicyDecision, SupplyChainPolicy};
fn make_component(license: License) -> Component {
    Component::new(
        "c1",
        "lib",
        "1.0",
        ComponentKind::Library,
        license,
        "vendor",
        "d1",
    )
}
#[test]
fn deny_gpl_returns_deny_for_gpl_component() {
    let policy = SupplyChainPolicy::new("t1").deny_license(License::Gpl3);
    let decision = policy.check_component(&make_component(License::Gpl3), true, true);
    assert!(matches!(decision, PolicyDecision::Deny(_)));
}
#[test]
fn deny_gpl_allows_mit_component() {
    let policy = SupplyChainPolicy::new("t1").deny_license(License::Gpl3);
    let decision = policy.check_component(&make_component(License::Mit), true, true);
    assert_eq!(decision, PolicyDecision::Allow);
}
