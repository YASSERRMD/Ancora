use crate::{
    CryptoKey, KeyAlgorithm, KeyAuditLog, KeyPurpose, KeyStatus, KeyStore,
    deactivate_key, compromise_key, destroy_key,
};
fn key(id: &str) -> CryptoKey {
    CryptoKey::new(id, "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m")
}
#[test]
fn deactivate_lifecycle_sets_inactive_and_records() {
    let mut store = KeyStore::new();
    let mut audit = KeyAuditLog::new();
    store.create(key("k1")).unwrap();
    deactivate_key(&mut store, "t1", "k1", "admin", 10, &mut audit).unwrap();
    assert_eq!(store.get_latest_mut("t1", "k1").unwrap().status, KeyStatus::Inactive);
    assert_eq!(audit.count(), 1);
}
#[test]
fn compromise_lifecycle_sets_compromised() {
    let mut store = KeyStore::new();
    let mut audit = KeyAuditLog::new();
    store.create(key("k2")).unwrap();
    compromise_key(&mut store, "t1", "k2", "sec-team", 20, &mut audit).unwrap();
    assert_eq!(store.get_latest_mut("t1", "k2").unwrap().status, KeyStatus::Compromised);
}
#[test]
fn destroy_lifecycle_clears_material() {
    let mut store = KeyStore::new();
    let mut audit = KeyAuditLog::new();
    store.create(key("k3")).unwrap();
    destroy_key(&mut store, "t1", "k3", "admin", 30, &mut audit).unwrap();
    let k = store.get_latest_mut("t1", "k3").unwrap();
    assert_eq!(k.status, KeyStatus::Destroyed);
    assert!(k.key_material.is_empty());
}
