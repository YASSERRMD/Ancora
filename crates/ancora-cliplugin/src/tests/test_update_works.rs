use std::collections::HashMap;

use crate::update::{RegistryEntry, UpdateRegistry, UpdateStatus, Version};

#[test]
fn test_version_parse() {
    let v = Version::parse("1.2.3").expect("should parse");
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);
}

#[test]
fn test_version_display() {
    let v = Version {
        major: 2,
        minor: 0,
        patch: 1,
    };
    assert_eq!(v.to_string_repr(), "2.0.1");
}

#[test]
fn test_version_ordering() {
    let a = Version::parse("1.0.0").unwrap();
    let b = Version::parse("2.0.0").unwrap();
    let c = Version::parse("1.0.0").unwrap();
    assert!(b > a);
    assert!(a == c);
}

#[test]
fn test_update_available_when_behind() {
    let mut registry = UpdateRegistry::new();
    registry.register(RegistryEntry {
        plugin_id: "my.plugin".to_string(),
        latest_version: Version::parse("2.0.0").unwrap(),
        update_url: Some("https://example.com/update".to_string()),
        notes: None,
    });

    let status = registry
        .check("my.plugin", "1.0.0")
        .expect("should find entry");
    assert!(
        matches!(status, UpdateStatus::UpdateAvailable(_)),
        "should report update available"
    );
}

#[test]
fn test_up_to_date_when_equal() {
    let mut registry = UpdateRegistry::new();
    registry.register(RegistryEntry {
        plugin_id: "my.plugin".to_string(),
        latest_version: Version::parse("1.5.0").unwrap(),
        update_url: None,
        notes: None,
    });

    let status = registry
        .check("my.plugin", "1.5.0")
        .expect("should find entry");
    assert!(
        matches!(status, UpdateStatus::UpToDate { .. }),
        "should report up to date"
    );
}

#[test]
fn test_ahead_of_registry_when_newer() {
    let mut registry = UpdateRegistry::new();
    registry.register(RegistryEntry {
        plugin_id: "my.plugin".to_string(),
        latest_version: Version::parse("1.0.0").unwrap(),
        update_url: None,
        notes: None,
    });

    let status = registry
        .check("my.plugin", "2.0.0")
        .expect("should find entry");
    assert!(
        matches!(status, UpdateStatus::AheadOfRegistry { .. }),
        "should report ahead of registry"
    );
}

#[test]
fn test_check_all() {
    let mut registry = UpdateRegistry::new();
    registry.register(RegistryEntry {
        plugin_id: "plug.a".to_string(),
        latest_version: Version::parse("2.0.0").unwrap(),
        update_url: None,
        notes: None,
    });
    registry.register(RegistryEntry {
        plugin_id: "plug.b".to_string(),
        latest_version: Version::parse("3.0.0").unwrap(),
        update_url: None,
        notes: None,
    });

    let mut installed = HashMap::new();
    installed.insert("plug.a".to_string(), "1.0.0".to_string());
    installed.insert("plug.b".to_string(), "3.0.0".to_string());

    let statuses = registry.check_all(&installed);
    assert_eq!(statuses.len(), 2);
}
