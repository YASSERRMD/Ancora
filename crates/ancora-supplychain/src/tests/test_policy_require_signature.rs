use crate::component::{Component, ComponentKind, License};
use crate::policy::{PolicyDecision, SupplyChainPolicy};
fn make_component() -> Component {
    Component::new("c1", "lib", "1.0", ComponentKind::Library, License::Mit, "vendor", "d1")
}
#[test]
fn require_signature_denies_unsigned_component() {
    let policy = SupplyChainPolicy::new("t1").require_signature();
    let decision = policy.check_component(&make_component(), false, true);
    assert!(matches!(decision, PolicyDecision::Deny(_)));
}
#[test]
fn require_signature_allows_signed_component() {
    let policy = SupplyChainPolicy::new("t1").require_signature();
    let decision = policy.check_component(&make_component(), true, true);
    assert_eq!(decision, PolicyDecision::Allow);
}
