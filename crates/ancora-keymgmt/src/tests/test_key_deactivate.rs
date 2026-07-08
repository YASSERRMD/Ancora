use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStatus};
#[test]
fn deactivate_changes_status() {
    let mut k = CryptoKey::new(
        "k1",
        "t1",
        KeyAlgorithm::Aes256,
        KeyPurpose::Encryption,
        0,
        "m",
    );
    k.deactivate();
    assert_eq!(k.status, KeyStatus::Inactive);
    assert!(!k.is_active());
}
