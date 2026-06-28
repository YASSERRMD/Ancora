use crate::{DefaultPosture, Effect, NetworkPolicy, NetworkRule, Protocol};
#[test]
fn rules_sorted_by_priority_ascending() {
    let mut policy = NetworkPolicy::new("t1", DefaultPosture::DenyAll);
    policy.add_rule(NetworkRule::new("low", "*", None, Protocol::Any, Effect::Deny, 200));
    policy.add_rule(NetworkRule::new("high", "*", None, Protocol::Any, Effect::Allow, 10));
    assert_eq!(policy.rules[0].id, "high");
    assert_eq!(policy.rules[1].id, "low");
}
