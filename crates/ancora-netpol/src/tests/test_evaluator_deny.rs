use crate::{
    ConnectionRequest, DefaultPosture, Effect, NetworkPolicy, NetworkRule, PolicyDecision,
    PolicyEvaluator, Protocol,
};
#[test]
fn evaluator_denies_matching_deny_rule() {
    let mut policy = NetworkPolicy::new("t1", DefaultPosture::AllowAll);
    policy.add_rule(NetworkRule::new(
        "deny-bad",
        "bad.example.com",
        None,
        Protocol::Any,
        Effect::Deny,
        10,
    ));
    let req = ConnectionRequest::tcp("t1", "a", "bad.example.com", 80);
    let decision = PolicyEvaluator::evaluate(&policy, &req);
    assert!(matches!(decision, PolicyDecision::Deny(_)));
}
