use crate::{CryptoKey, KeyAlgorithm, KeyPurpose};
#[test]
fn key_expiry_check() {
    let k = CryptoKey::new(
        "k1",
        "t1",
        KeyAlgorithm::Aes256,
        KeyPurpose::Encryption,
        0,
        "m",
    )
    .with_expiry(100);
    assert!(!k.is_expired(50));
    assert!(k.is_expired(100));
    assert!(k.is_expired(200));
}
#[test]
fn key_without_expiry_never_expires() {
    let k = CryptoKey::new(
        "k1",
        "t1",
        KeyAlgorithm::Aes256,
        KeyPurpose::Encryption,
        0,
        "m",
    );
    assert!(!k.is_expired(u64::MAX));
}
