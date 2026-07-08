use crate::{DefaultPosture, Effect, NetworkPolicy, NetworkRule, Protocol};
#[test]
fn allow_rules_filters_correctly() {
    let mut policy = NetworkPolicy::new("t1", DefaultPosture::DenyAll);
    policy.add_rule(NetworkRule::new(
        "a1",
        "*",
        Some(443),
        Protocol::Tcp,
        Effect::Allow,
        100,
    ));
    policy.add_rule(NetworkRule::new(
        "d1",
        "bad.com",
        None,
        Protocol::Any,
        Effect::Deny,
        10,
    ));
    assert_eq!(policy.allow_rules().len(), 1);
    assert_eq!(policy.deny_rules().len(), 1);
}
