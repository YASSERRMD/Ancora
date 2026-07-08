use crate::{
    ConnectionRequest, DefaultPosture, Effect, NetworkPolicy, NetworkRule, PolicyDecision,
    PolicyEvaluator, Protocol,
};
#[test]
fn lower_priority_deny_wins_over_higher_allow() {
    let mut policy = NetworkPolicy::new("t1", DefaultPosture::AllowAll);
    policy.add_rule(NetworkRule::new(
        "deny-80",
        "*",
        Some(80),
        Protocol::Tcp,
        Effect::Deny,
        5,
    ));
    policy.add_rule(NetworkRule::new(
        "allow-all",
        "*",
        None,
        Protocol::Any,
        Effect::Allow,
        100,
    ));
    let req = ConnectionRequest::tcp("t1", "a", "example.com", 80);
    assert!(matches!(
        PolicyEvaluator::evaluate(&policy, &req),
        PolicyDecision::Deny(_)
    ));
}
