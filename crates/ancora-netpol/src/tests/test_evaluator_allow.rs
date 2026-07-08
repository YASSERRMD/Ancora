use crate::{
    ConnectionRequest, DefaultPosture, Effect, NetworkPolicy, NetworkRule, PolicyDecision,
    PolicyEvaluator, Protocol,
};
#[test]
fn evaluator_allows_matching_allow_rule() {
    let mut policy = NetworkPolicy::new("t1", DefaultPosture::DenyAll);
    policy.add_rule(NetworkRule::new(
        "allow-443",
        "*",
        Some(443),
        Protocol::Tcp,
        Effect::Allow,
        100,
    ));
    let req = ConnectionRequest::tcp("t1", "a", "api.com", 443);
    assert_eq!(
        PolicyEvaluator::evaluate(&policy, &req),
        PolicyDecision::Allow
    );
}
