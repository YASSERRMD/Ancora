/// Tests for data-residency enforcement.
///
/// The residency policy controls which geographic regions a plugin is allowed
/// to send data to or receive data from.  This is relevant for GDPR, CCPA,
/// and other data-sovereignty requirements.
use crate::residency::{ResidencyPolicy, ResidencyViolation};

#[test]
fn data_within_allowed_region_permitted() {
    let policy = ResidencyPolicy::allow_only(vec!["eu-west".into(), "us-east".into()]);
    assert!(policy.permits_transfer("eu-west").is_ok());
    assert!(policy.permits_transfer("us-east").is_ok());
}

#[test]
fn data_outside_allowed_region_rejected() {
    let policy = ResidencyPolicy::allow_only(vec!["eu-west".into()]);
    let err = policy.permits_transfer("ap-southeast").unwrap_err();
    assert!(matches!(err, ResidencyViolation::RegionNotPermitted { .. }));
}

#[test]
fn global_policy_permits_all_regions() {
    let policy = ResidencyPolicy::global();
    assert!(policy.permits_transfer("eu-west").is_ok());
    assert!(policy.permits_transfer("us-east").is_ok());
    assert!(policy.permits_transfer("ap-southeast").is_ok());
}

#[test]
fn deny_all_policy_blocks_all_regions() {
    let policy = ResidencyPolicy::deny_all();
    assert!(policy.permits_transfer("eu-west").is_err());
    assert!(policy.permits_transfer("us-east").is_err());
}

#[test]
fn residency_violation_contains_region_name() {
    let policy = ResidencyPolicy::allow_only(vec!["eu-west".into()]);
    let err = policy.permits_transfer("cn-north").unwrap_err();
    match err {
        ResidencyViolation::RegionNotPermitted { region } => {
            assert_eq!(region, "cn-north");
        }
    }
}
