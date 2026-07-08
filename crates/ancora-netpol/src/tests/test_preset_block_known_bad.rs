use crate::{presets, ConnectionRequest, NetworkPolicy, PolicyDecision, PolicyEvaluator};
#[test]
fn block_known_bad_denies_that_host() {
    let mut policy = NetworkPolicy::allow_by_default("t1");
    presets::block_known_bad(&mut policy, "malware.com");
    let req = ConnectionRequest::tcp("t1", "a", "malware.com", 443);
    assert!(matches!(
        PolicyEvaluator::evaluate(&policy, &req),
        PolicyDecision::Deny(_)
    ));
}
