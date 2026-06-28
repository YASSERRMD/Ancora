use crate::publish::PublishEntry;
use crate::service::{RegistryConfig, RegistryService};
use crate::versioning::Version;

#[test]
fn versions_are_listed_in_ascending_order() {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let versions = [
        Version::new(1, 3, 0),
        Version::new(1, 0, 0),
        Version::new(2, 0, 0),
        Version::new(1, 1, 0),
    ];
    for v in &versions {
        svc.publish(PublishEntry::new("tool", v.clone(), b"data".to_vec(), "ci"))
            .unwrap();
    }

    let listed = svc.list_versions("tool");
    assert_eq!(listed.len(), 4);
    // Should be sorted ascending.
    assert_eq!(listed[0], Version::new(1, 0, 0));
    assert_eq!(listed[1], Version::new(1, 1, 0));
    assert_eq!(listed[2], Version::new(1, 3, 0));
    assert_eq!(listed[3], Version::new(2, 0, 0));
}

#[test]
fn duplicate_version_not_listed_twice() {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let v = Version::new(1, 0, 0);
    svc.publish(PublishEntry::new("tool", v.clone(), b"first".to_vec(), "ci"))
        .unwrap();
    // Second publish of same version overwrites payload but does not duplicate the version entry.
    svc.publish(PublishEntry::new("tool", v.clone(), b"second".to_vec(), "ci"))
        .unwrap();

    let listed = svc.list_versions("tool");
    assert_eq!(listed.len(), 1);
}

#[test]
fn versions_of_unknown_entry_is_empty() {
    let svc = RegistryService::new(RegistryConfig::default());
    assert!(svc.list_versions("ghost").is_empty());
}
