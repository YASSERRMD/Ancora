use crate::evaluator::ZeroTrustEvaluator;
use crate::identity::{Identity, IdentityKind};
use crate::device::DeviceStore;
use crate::request::AccessRequest;
use crate::policy::{AuthzDecision, ZeroTrustPolicy};

#[test]
fn require_mfa_for_admin_group() {
    let policy = ZeroTrustPolicy::new("t1").mfa_for_group("admin");
    let mut identity = Identity::new("i1", "t1", IdentityKind::Human, 1);
    identity.add_group("admin");
    let request = AccessRequest::new("r1", "t1", "i1", "admin/users", "GET", 1);
    let devices = DeviceStore::new();
    assert_eq!(ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices), AuthzDecision::RequireMfa);
}

#[test]
fn no_mfa_for_non_admin_group() {
    let policy = ZeroTrustPolicy::new("t1").mfa_for_group("admin");
    let mut identity = Identity::new("i1", "t1", IdentityKind::Human, 1);
    identity.add_group("user");
    let request = AccessRequest::new("r1", "t1", "i1", "api", "GET", 1);
    let devices = DeviceStore::new();
    assert_eq!(ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices), AuthzDecision::Allow);
}
