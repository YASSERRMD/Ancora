use crate::{ConnectionRequest, PolicyDecision, PolicyEvaluator, presets};
#[test]
fn internal_only_allows_internal_host() {
    let policy = presets::allow_internal_only("t1", "internal.corp");
    let req = ConnectionRequest::tcp("t1", "a", "api.internal.corp", 8080);
    assert_eq!(PolicyEvaluator::evaluate(&policy, &req), PolicyDecision::Allow);
}
#[test]
fn internal_only_denies_external_host() {
    let policy = presets::allow_internal_only("t1", "internal.corp");
    let req = ConnectionRequest::tcp("t1", "a", "api.external.com", 443);
    assert!(matches!(PolicyEvaluator::evaluate(&policy, &req), PolicyDecision::Deny(_)));
}
