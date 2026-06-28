use crate::{Effect, NetworkRule, Protocol};
#[test]
fn exact_host_matches() {
    let r = NetworkRule::new("r1", "example.com", None, Protocol::Tcp, Effect::Allow, 100);
    assert!(r.matches_host("example.com"));
    assert!(!r.matches_host("other.com"));
}
