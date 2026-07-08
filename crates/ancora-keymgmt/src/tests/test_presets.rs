use crate::{
    aes256_encryption_key, ed25519_signing_key, ephemeral_key, KeyAlgorithm, KeyPurpose, KeyStatus,
};
#[test]
fn aes256_preset_is_active_encryption() {
    let k = aes256_encryption_key("k1", "t1", 10);
    assert_eq!(k.algorithm, KeyAlgorithm::Aes256);
    assert_eq!(k.purpose, KeyPurpose::Encryption);
    assert_eq!(k.status, KeyStatus::Active);
}
#[test]
fn ed25519_preset_is_signing() {
    let k = ed25519_signing_key("k2", "t1", 0);
    assert_eq!(k.algorithm, KeyAlgorithm::Ed25519);
    assert_eq!(k.purpose, KeyPurpose::Signing);
}
#[test]
fn ephemeral_preset_has_expiry() {
    let k = ephemeral_key("k3", "t1", 100, 50);
    assert_eq!(k.expires_tick, Some(150));
    assert!(k.is_expired(150));
    assert!(!k.is_expired(149));
}
