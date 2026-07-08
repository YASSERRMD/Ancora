use crate::{
    ConnectionRequest, DefaultPosture, Effect, NetworkPolicy, NetworkRule, PolicyDecision,
    PolicyEvaluator, Protocol,
};
#[test]
fn no_matching_rule_falls_through_to_default() {
    let mut policy = NetworkPolicy::new("t1", DefaultPosture::AllowAll);
    policy.add_rule(NetworkRule::new(
        "r1",
        "specific.com",
        Some(443),
        Protocol::Tcp,
        Effect::Allow,
        100,
    ));
    let req = ConnectionRequest::tcp("t1", "a", "other.com", 443);
    assert_eq!(
        PolicyEvaluator::evaluate(&policy, &req),
        PolicyDecision::Allow
    );
}
