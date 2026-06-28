use crate::{Effect, NetworkRule, Protocol};
#[test]
fn any_protocol_matches_tcp_and_udp() {
    let r = NetworkRule::new("r1", "*", None, Protocol::Any, Effect::Allow, 100);
    assert!(r.matches_protocol(&Protocol::Tcp));
    assert!(r.matches_protocol(&Protocol::Udp));
    assert!(r.matches_protocol(&Protocol::Any));
}
#[test]
fn tcp_rule_does_not_match_udp() {
    let r = NetworkRule::new("r1", "*", None, Protocol::Tcp, Effect::Allow, 100);
    assert!(!r.matches_protocol(&Protocol::Udp));
}
