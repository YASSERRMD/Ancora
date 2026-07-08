use crate::storage::{EncryptedLocalStorage, StorageKey};

#[test]
fn test_local_storage_encrypted() {
    let key = StorageKey::from_device_id("device-xor-test");
    let mut store = EncryptedLocalStorage::new(key.clone());
    let plaintext = b"secret-agent-data";
    store.put("key1", plaintext);
    // Ciphertext should differ from plaintext.
    let ct = store.get_ciphertext("key1").unwrap();
    assert_ne!(ct, plaintext.as_ref(), "data at rest should be encrypted");
}

#[test]
fn test_local_storage_roundtrip() {
    let key = StorageKey::from_device_id("device-roundtrip");
    let mut store = EncryptedLocalStorage::new(key);
    let plaintext = b"hello world from edge";
    store.put("data", plaintext);
    let recovered = store.get("data").unwrap();
    assert_eq!(
        recovered,
        plaintext.to_vec(),
        "roundtrip should recover plaintext"
    );
}

#[test]
fn test_local_storage_missing_key() {
    let key = StorageKey::from_device_id("device-missing");
    let store = EncryptedLocalStorage::new(key);
    assert!(store.get("no-such-key").is_none());
}

#[test]
fn test_local_storage_remove() {
    let key = StorageKey::from_device_id("device-remove");
    let mut store = EncryptedLocalStorage::new(key);
    store.put("to-remove", b"data");
    assert!(store.contains_key("to-remove"));
    store.remove("to-remove");
    assert!(!store.contains_key("to-remove"));
}

#[test]
fn test_storage_key_deterministic() {
    let k1 = StorageKey::from_device_id("dev-42");
    let k2 = StorageKey::from_device_id("dev-42");
    assert_eq!(k1.bytes, k2.bytes);
}
