use crate::presets::{permissive_policy, standard_policy, strict_policy};
use crate::device::TrustLevel;

#[test]
fn strict_policy_requires_device_and_mfa() {
    let p = strict_policy("t1");
    assert!(p.require_device_trust);
    assert_eq!(p.min_device_trust, TrustLevel::FullyTrusted);
    assert!(p.needs_mfa_for(&["admin".to_string()]));
}

#[test]
fn standard_policy_requires_trusted() {
    let p = standard_policy("t1");
    assert!(p.require_device_trust);
    assert_eq!(p.min_device_trust, TrustLevel::Trusted);
}

#[test]
fn permissive_policy_allows_all() {
    let p = permissive_policy("t1");
    assert!(!p.require_device_trust);
    assert!(!p.needs_mfa_for(&["admin".to_string()]));
}
