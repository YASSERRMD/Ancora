use crate::{Effect, Protocol, RuleBuilder};
#[test]
fn builder_sets_all_fields() {
    let rule = RuleBuilder::new("r1")
        .host("api.example.com")
        .port(443)
        .tcp()
        .allow()
        .priority(50)
        .description("allow api")
        .build();
    assert_eq!(rule.id, "r1");
    assert_eq!(rule.host_pattern, "api.example.com");
    assert_eq!(rule.port, Some(443));
    assert_eq!(rule.protocol, Protocol::Tcp);
    assert_eq!(rule.effect, Effect::Allow);
    assert_eq!(rule.priority, 50);
    assert_eq!(rule.description, "allow api");
}
#[test]
fn builder_defaults_to_allow_any() {
    let rule = RuleBuilder::new("default").build();
    assert_eq!(rule.effect, Effect::Allow);
    assert_eq!(rule.protocol, Protocol::Any);
    assert_eq!(rule.host_pattern, "*");
    assert!(rule.port.is_none());
}
#[test]
fn builder_deny_rule() {
    let rule = RuleBuilder::new("block").host("bad.com").deny().build();
    assert_eq!(rule.effect, Effect::Deny);
}
