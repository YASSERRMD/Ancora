use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStatus};
#[test]
fn mark_compromised_changes_status() {
    let mut k = CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m");
    k.mark_compromised();
    assert_eq!(k.status, KeyStatus::Compromised);
}
