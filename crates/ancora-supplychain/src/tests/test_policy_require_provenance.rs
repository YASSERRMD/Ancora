use crate::component::{Component, ComponentKind, License};
use crate::policy::{PolicyDecision, SupplyChainPolicy};
fn make_component() -> Component {
    Component::new(
        "c1",
        "lib",
        "1.0",
        ComponentKind::Library,
        License::Mit,
        "vendor",
        "d1",
    )
}
#[test]
fn require_provenance_denies_component_without_provenance() {
    let policy = SupplyChainPolicy::new("t1").require_provenance();
    let decision = policy.check_component(&make_component(), true, false);
    assert!(matches!(decision, PolicyDecision::Deny(_)));
}
#[test]
fn require_provenance_allows_component_with_provenance() {
    let policy = SupplyChainPolicy::new("t1").require_provenance();
    let decision = policy.check_component(&make_component(), true, true);
    assert_eq!(decision, PolicyDecision::Allow);
}
