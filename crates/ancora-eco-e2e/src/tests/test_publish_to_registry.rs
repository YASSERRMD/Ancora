use crate::registry_e2e::{LocalRegistry, RegistryEntry};

#[test]
fn test_publish_plugin_to_registry() {
    let mut registry = LocalRegistry::new();
    let entry = RegistryEntry::new("hello-plugin", "0.1.0", "acme");
    registry.publish(entry).expect("publish must succeed");
    assert!(registry.latest("hello-plugin").is_some());
}

#[test]
fn test_publish_duplicate_version_fails() {
    let mut registry = LocalRegistry::new();
    let entry = RegistryEntry::new("dup-plugin", "1.0.0", "acme");
    let entry2 = RegistryEntry::new("dup-plugin", "1.0.0", "acme");
    registry.publish(entry).unwrap();
    let result = registry.publish(entry2);
    assert!(result.is_err());
}

#[test]
fn test_publish_multiple_versions() {
    let mut registry = LocalRegistry::new();
    registry
        .publish(RegistryEntry::new("versioned", "0.1.0", "org"))
        .unwrap();
    registry
        .publish(RegistryEntry::new("versioned", "0.2.0", "org"))
        .unwrap();
    let versions = registry.all_versions("versioned");
    assert_eq!(versions.len(), 2);
    let latest = registry.latest("versioned").unwrap();
    assert_eq!(latest.version, "0.2.0");
}

#[test]
fn test_list_all_published() {
    let mut registry = LocalRegistry::new();
    registry
        .publish(RegistryEntry::new("alpha", "1.0.0", "org"))
        .unwrap();
    registry
        .publish(RegistryEntry::new("beta", "1.0.0", "org"))
        .unwrap();
    assert_eq!(registry.list_all().len(), 2);
}
