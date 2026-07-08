use crate::access_control::AccessPolicy;
use crate::publish::{PublishEntry, PublishError};
use crate::service::{RegistryConfig, RegistryService};
use crate::versioning::Version;

#[test]
fn allowed_publisher_can_publish() {
    let cfg = RegistryConfig {
        access_policy: AccessPolicy::allow_list(["trusted-ci"]),
        ..Default::default()
    };
    let mut svc = RegistryService::new(cfg);

    let entry = PublishEntry::new(
        "tool",
        Version::new(1, 0, 0),
        b"data".to_vec(),
        "trusted-ci",
    );
    svc.publish(entry)
        .expect("allowed publisher should succeed");
}

#[test]
fn unlisted_publisher_is_denied() {
    let cfg = RegistryConfig {
        access_policy: AccessPolicy::allow_list(["trusted-ci"]),
        ..Default::default()
    };
    let mut svc = RegistryService::new(cfg);

    let entry = PublishEntry::new("tool", Version::new(1, 0, 0), b"data".to_vec(), "attacker");
    let err = svc.publish(entry).unwrap_err();
    assert!(matches!(err, PublishError::AccessDenied(_)));
}

#[test]
fn deny_all_policy_rejects_every_publisher() {
    let cfg = RegistryConfig {
        access_policy: AccessPolicy::DenyAll,
        ..Default::default()
    };
    let mut svc = RegistryService::new(cfg);

    let entry = PublishEntry::new("tool", Version::new(1, 0, 0), b"data".to_vec(), "admin");
    let err = svc.publish(entry).unwrap_err();
    assert!(matches!(err, PublishError::AccessDenied(_)));
}

#[test]
fn open_policy_accepts_any_publisher() {
    let cfg = RegistryConfig {
        access_policy: AccessPolicy::Open,
        ..Default::default()
    };
    let mut svc = RegistryService::new(cfg);
    let entry = PublishEntry::new("tool", Version::new(1, 0, 0), b"data".to_vec(), "anyone");
    svc.publish(entry)
        .expect("open policy should accept anyone");
}
