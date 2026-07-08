use crate::{presets, ConnectionRequest, PolicyDecision, PolicyEvaluator};
#[test]
fn https_only_allows_port_443() {
    let policy = presets::allow_https_only("t1");
    let req = ConnectionRequest::tcp("t1", "a", "api.example.com", 443);
    assert_eq!(
        PolicyEvaluator::evaluate(&policy, &req),
        PolicyDecision::Allow
    );
}
#[test]
fn https_only_denies_port_80() {
    let policy = presets::allow_https_only("t1");
    let req = ConnectionRequest::tcp("t1", "a", "api.example.com", 80);
    assert!(matches!(
        PolicyEvaluator::evaluate(&policy, &req),
        PolicyDecision::Deny(_)
    ));
}
