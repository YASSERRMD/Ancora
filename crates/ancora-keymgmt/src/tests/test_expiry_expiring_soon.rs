use crate::{CryptoKey, ExpiryChecker, KeyAlgorithm, KeyPurpose, KeyStore};
#[test]
fn expiring_soon_returns_keys_within_warning_window() {
    let mut store = KeyStore::new();
    store
        .create(
            CryptoKey::new(
                "k1",
                "t1",
                KeyAlgorithm::Aes256,
                KeyPurpose::Encryption,
                0,
                "m",
            )
            .with_expiry(110),
        )
        .unwrap();
    store
        .create(
            CryptoKey::new(
                "k2",
                "t1",
                KeyAlgorithm::Aes256,
                KeyPurpose::Encryption,
                0,
                "m",
            )
            .with_expiry(200),
        )
        .unwrap();
    let soon = ExpiryChecker::expiring_soon(&store, "t1", 100, 20);
    assert_eq!(soon.len(), 1);
    assert_eq!(soon[0].id, "k1");
}
