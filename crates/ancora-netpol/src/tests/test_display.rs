use crate::{Effect, Protocol, RuleBuilder};
#[test]
fn protocol_display() {
    assert_eq!(format!("{}", Protocol::Tcp), "TCP");
    assert_eq!(format!("{}", Protocol::Udp), "UDP");
    assert_eq!(format!("{}", Protocol::Any), "ANY");
}
#[test]
fn effect_display() {
    assert_eq!(format!("{}", Effect::Allow), "ALLOW");
    assert_eq!(format!("{}", Effect::Deny), "DENY");
}
#[test]
fn rule_display_includes_key_fields() {
    let rule = RuleBuilder::new("r1").host("api.com").port(443).tcp().allow().priority(10).build();
    let s = format!("{}", rule);
    assert!(s.contains("r1"));
    assert!(s.contains("api.com"));
    assert!(s.contains("443"));
    assert!(s.contains("TCP"));
    assert!(s.contains("ALLOW"));
    assert!(s.contains("10"));
}
