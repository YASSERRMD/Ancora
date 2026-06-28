use crate::KeyAlgorithm;
#[test]
fn algorithm_display() {
    assert_eq!(format!("{}", KeyAlgorithm::Aes256), "AES-256");
    assert_eq!(format!("{}", KeyAlgorithm::Rsa2048), "RSA-2048");
    assert_eq!(format!("{}", KeyAlgorithm::Ed25519), "ED25519");
}
