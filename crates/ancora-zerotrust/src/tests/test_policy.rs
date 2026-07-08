use crate::device::TrustLevel;
use crate::policy::ZeroTrustPolicy;

#[test]
fn default_policy_no_device_required() {
    let p = ZeroTrustPolicy::new("t1");
    assert!(!p.require_device_trust);
    assert!(!p.resource_denied("anything"));
}

#[test]
fn policy_deny_resource() {
    let p = ZeroTrustPolicy::new("t1").deny_resource("admin/secrets");
    assert!(p.resource_denied("admin/secrets"));
    assert!(!p.resource_denied("other"));
}

#[test]
fn policy_mfa_for_group() {
    let p = ZeroTrustPolicy::new("t1").mfa_for_group("admin");
    assert!(p.needs_mfa_for(&["admin".to_string()]));
    assert!(!p.needs_mfa_for(&["user".to_string()]));
}

#[test]
fn policy_require_device() {
    let p = ZeroTrustPolicy::new("t1")
        .require_device()
        .min_trust(TrustLevel::Trusted);
    assert!(p.require_device_trust);
    assert_eq!(p.min_device_trust, TrustLevel::Trusted);
}
