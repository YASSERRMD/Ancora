use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStatus};
#[test]
fn new_key_defaults_to_active_version_one() {
    let k = CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 10, "material");
    assert_eq!(k.status, KeyStatus::Active);
    assert_eq!(k.version, 1);
    assert!(k.is_active());
    assert_eq!(k.created_tick, 10);
}
