use crate::evaluator::ZeroTrustEvaluator;
use crate::identity::{Identity, IdentityKind};
use crate::device::{DevicePosture, DeviceStore};
use crate::request::AccessRequest;
use crate::policy::{AuthzDecision, ZeroTrustPolicy};

fn make_trusted_device(id: &str, tenant_id: &str, tick: u64) -> DevicePosture {
    let mut d = DevicePosture::new(id, tenant_id, tick);
    d.os_up_to_date = true;
    d.antivirus_active = true;
    d.disk_encrypted = true;
    d.compute_trust();
    d
}

#[test]
fn allow_simple_request() {
    let policy = ZeroTrustPolicy::new("t1");
    let identity = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let request = AccessRequest::new("r1", "t1", "i1", "api/data", "GET", 100);
    let devices = DeviceStore::new();
    let decision = ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices);
    assert_eq!(decision, AuthzDecision::Allow);
}

#[test]
fn deny_suspended_identity() {
    let policy = ZeroTrustPolicy::new("t1");
    let mut identity = Identity::new("i1", "t1", IdentityKind::Human, 1);
    identity.suspend();
    let request = AccessRequest::new("r1", "t1", "i1", "api", "GET", 1);
    let devices = DeviceStore::new();
    assert!(matches!(ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices), AuthzDecision::Deny(_)));
}

#[test]
fn deny_blocked_resource() {
    let policy = ZeroTrustPolicy::new("t1").deny_resource("admin/secrets");
    let identity = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let request = AccessRequest::new("r1", "t1", "i1", "admin/secrets", "GET", 1);
    let devices = DeviceStore::new();
    assert!(matches!(ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices), AuthzDecision::Deny(_)));
}
