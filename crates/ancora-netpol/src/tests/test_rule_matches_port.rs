use crate::{Effect, NetworkRule, Protocol};
#[test]
fn port_match_works() {
    let r = NetworkRule::new("r1", "*", Some(443), Protocol::Tcp, Effect::Allow, 100);
    assert!(r.matches_port(443));
    assert!(!r.matches_port(80));
}
#[test]
fn no_port_restriction_matches_any_port() {
    let r = NetworkRule::new("r1", "*", None, Protocol::Tcp, Effect::Allow, 100);
    assert!(r.matches_port(80));
    assert!(r.matches_port(65535));
}
