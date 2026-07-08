use crate::{ConnectionRequest, NetworkPolicy, PolicyDecision, PolicyEvaluator};
#[test]
fn no_matching_rule_with_allow_default_is_allowed() {
    let policy = NetworkPolicy::allow_by_default("t1");
    let req = ConnectionRequest::tcp("t1", "a", "anywhere.com", 9000);
    assert_eq!(
        PolicyEvaluator::evaluate(&policy, &req),
        PolicyDecision::Allow
    );
}
