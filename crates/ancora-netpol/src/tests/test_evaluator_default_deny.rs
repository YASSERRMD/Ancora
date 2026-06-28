use crate::{ConnectionRequest, NetworkPolicy, PolicyDecision, PolicyEvaluator};
#[test]
fn no_matching_rule_with_deny_default_is_denied() {
    let policy = NetworkPolicy::deny_by_default("t1");
    let req = ConnectionRequest::tcp("t1", "a", "external.com", 80);
    let decision = PolicyEvaluator::evaluate(&policy, &req);
    assert!(matches!(decision, PolicyDecision::Deny(_)));
}
