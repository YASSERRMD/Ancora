use crate::registration::*;
use std::collections::HashMap;

#[test]
fn test_device_registers() {
    let mut registry = DeviceRegistry::new();
    let req = RegistrationRequest {
        device_id: DeviceId::new("dev-001"),
        name: "Edge Node Alpha".into(),
        fingerprint: "fp-abc123".into(),
        metadata: HashMap::new(),
    };
    let resp = registry.register(req);
    assert!(resp.success);
    assert!(resp.token.is_some());
    assert_eq!(registry.count(), 1);
}

#[test]
fn test_duplicate_registration_rejected() {
    let mut registry = DeviceRegistry::new();
    let req1 = RegistrationRequest {
        device_id: DeviceId::new("dev-001"),
        name: "Edge Node Alpha".into(),
        fingerprint: "fp-abc123".into(),
        metadata: HashMap::new(),
    };
    let req2 = RegistrationRequest {
        device_id: DeviceId::new("dev-001"),
        name: "Dup".into(),
        fingerprint: "fp-dup".into(),
        metadata: HashMap::new(),
    };
    registry.register(req1);
    let resp = registry.register(req2);
    assert!(!resp.success);
    assert!(resp.token.is_none());
}

#[test]
fn test_device_identity_activate_revoke() {
    let id = DeviceId::new("d1");
    let mut identity = DeviceIdentity::new(id, "Test", "fp");
    assert_eq!(identity.status, RegistrationStatus::Pending);
    identity.activate();
    assert!(identity.is_active());
    identity.revoke();
    assert!(!identity.is_active());
    assert_eq!(identity.status, RegistrationStatus::Revoked);
}

#[test]
fn test_active_devices_filtered() {
    let mut registry = DeviceRegistry::new();
    for i in 0..3 {
        let req = RegistrationRequest {
            device_id: DeviceId::new(format!("dev-{}", i)),
            name: format!("Node {}", i),
            fingerprint: format!("fp-{}", i),
            metadata: HashMap::new(),
        };
        registry.register(req);
    }
    assert_eq!(registry.active_devices().len(), 3);
}
