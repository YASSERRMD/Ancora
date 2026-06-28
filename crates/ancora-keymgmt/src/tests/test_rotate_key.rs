use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStatus, KeyStore, rotate_key};
#[test]
fn rotate_key_deactivates_previous_and_creates_new_active() {
    let mut store = KeyStore::new();
    store.create(CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "v1")).unwrap();
    rotate_key(&mut store, "t1", "k1", "v2", 10).unwrap();
    let active = store.get_active("t1", "k1").unwrap();
    assert_eq!(active.version, 2);
    let v1 = store.get_version("t1", "k1", 1).unwrap();
    assert_eq!(v1.status, KeyStatus::Inactive);
}
