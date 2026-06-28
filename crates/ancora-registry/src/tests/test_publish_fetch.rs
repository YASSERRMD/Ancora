use crate::fetch::FetchResult;
use crate::publish::PublishEntry;
use crate::service::{RegistryConfig, RegistryService};
use crate::versioning::Version;

#[test]
fn published_entry_is_fetchable() {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let version = Version::new(1, 0, 0);
    let payload = b"hello-registry".to_vec();
    let entry = PublishEntry::new("my-tool", version.clone(), payload.clone(), "alice");

    svc.publish(entry).expect("publish should succeed");

    let result = svc.fetch("my-tool", &version);
    assert_eq!(result, FetchResult::Found(payload));
}

#[test]
fn missing_entry_returns_not_found() {
    let svc = RegistryService::new(RegistryConfig::default());
    let result = svc.fetch("nonexistent", &Version::new(0, 1, 0));
    assert_eq!(result, FetchResult::NotFound);
}

#[test]
fn multiple_versions_are_independently_fetchable() {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    svc.publish(PublishEntry::new("tool", v1.clone(), b"v1-data".to_vec(), "bob"))
        .unwrap();
    svc.publish(PublishEntry::new("tool", v2.clone(), b"v2-data".to_vec(), "bob"))
        .unwrap();

    assert_eq!(svc.fetch("tool", &v1), FetchResult::Found(b"v1-data".to_vec()));
    assert_eq!(svc.fetch("tool", &v2), FetchResult::Found(b"v2-data".to_vec()));
}
