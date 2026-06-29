use crate::identity::{DeviceId, DeviceIdentityRegistry, DeviceKeyPair};

#[test]
fn test_device_identity_verified() {
    let mut registry = DeviceIdentityRegistry::new();
    let id = DeviceId::new("edge-device-001");
    let kp = registry.register(id.clone());
    let pub_key = kp.public_key.clone();
    assert!(registry.verify_identity(&id, &pub_key), "identity should verify with correct public key");
}

#[test]
fn test_device_identity_wrong_key_rejected() {
    let mut registry = DeviceIdentityRegistry::new();
    let id = DeviceId::new("edge-device-002");
    registry.register(id.clone());
    let wrong_key = vec![0u8; 32];
    assert!(!registry.verify_identity(&id, &wrong_key), "wrong key should not verify");
}

#[test]
fn test_device_key_deterministic() {
    let id = DeviceId::new("det-device");
    let kp1 = DeviceKeyPair::generate(id.clone());
    let kp2 = DeviceKeyPair::generate(id.clone());
    assert_eq!(kp1.public_key, kp2.public_key, "keys should be deterministic");
}

#[test]
fn test_device_key_hex_length() {
    let id = DeviceId::new("hex-test-device");
    let kp = DeviceKeyPair::generate(id);
    assert_eq!(kp.public_key_hex().len(), 64, "32 bytes = 64 hex chars");
}

#[test]
fn test_device_registry_revoke() {
    let mut registry = DeviceIdentityRegistry::new();
    let id = DeviceId::new("revoke-device");
    let kp = registry.register(id.clone());
    let pub_key = kp.public_key.clone();
    assert!(registry.verify_identity(&id, &pub_key));
    registry.revoke(&id);
    assert!(!registry.verify_identity(&id, &pub_key), "revoked device should not verify");
    assert!(registry.is_revoked(&id));
}

#[test]
fn test_device_registry_unregistered() {
    let registry = DeviceIdentityRegistry::new();
    let id = DeviceId::new("unknown-device");
    assert!(!registry.verify_identity(&id, &[0u8; 32]));
    assert!(!registry.is_revoked(&id));
}
