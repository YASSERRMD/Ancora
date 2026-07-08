use crate::device::TrustLevel;
use crate::device::{DevicePosture, DeviceStore};
use crate::evaluator::ZeroTrustEvaluator;
use crate::identity::{Identity, IdentityKind};
use crate::policy::{AuthzDecision, ZeroTrustPolicy};
use crate::request::AccessRequest;

#[test]
fn deny_when_device_required_but_not_presented() {
    let policy = ZeroTrustPolicy::new("t1").require_device();
    let identity = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let request = AccessRequest::new("r1", "t1", "i1", "api", "GET", 1);
    let devices = DeviceStore::new();
    assert!(matches!(
        ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices),
        AuthzDecision::Deny(_)
    ));
}

#[test]
fn allow_with_trusted_device() {
    let policy = ZeroTrustPolicy::new("t1")
        .require_device()
        .min_trust(TrustLevel::Trusted);
    let identity = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let request = AccessRequest::new("r1", "t1", "i1", "api", "GET", 1).with_device("d1");
    let mut devices = DeviceStore::new();
    let mut d = DevicePosture::new("d1", "t1", 1);
    d.os_up_to_date = true;
    d.antivirus_active = true;
    d.disk_encrypted = true;
    d.compute_trust();
    devices.upsert(d);
    assert_eq!(
        ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices),
        AuthzDecision::Allow
    );
}
