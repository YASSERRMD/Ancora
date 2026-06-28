use crate::{DefaultPosture, Effect, NetworkPolicy, NetworkRule, Protocol};
#[test]
fn policy_stores_rule_and_increments_count() {
    let mut policy = NetworkPolicy::new("t1", DefaultPosture::DenyAll);
    policy.add_rule(NetworkRule::new("r1", "*", Some(443), Protocol::Tcp, Effect::Allow, 100));
    assert_eq!(policy.rule_count(), 1);
}
