use crate::{Effect, NetworkRule, Protocol};
#[test]
fn star_matches_any_host() {
    let r = NetworkRule::new("r1", "*", None, Protocol::Any, Effect::Allow, 100);
    assert!(r.matches_host("anything.example.com"));
    assert!(r.matches_host("192.168.0.1"));
}
#[test]
fn wildcard_subdomain_matches() {
    let r = NetworkRule::new("r1", "*.internal.example.com", None, Protocol::Any, Effect::Allow, 100);
    assert!(r.matches_host("api.internal.example.com"));
    assert!(r.matches_host("db.internal.example.com"));
    assert!(!r.matches_host("external.example.com"));
}
