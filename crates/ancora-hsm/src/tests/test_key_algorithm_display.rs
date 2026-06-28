use crate::key::HsmKeyAlgorithm;

#[test]
fn display_aes128() {
    assert_eq!(format!("{}", HsmKeyAlgorithm::Aes128), "AES-128");
}

#[test]
fn display_aes256() {
    assert_eq!(format!("{}", HsmKeyAlgorithm::Aes256), "AES-256");
}

#[test]
fn display_rsa2048() {
    assert_eq!(format!("{}", HsmKeyAlgorithm::Rsa2048), "RSA-2048");
}

#[test]
fn display_rsa4096() {
    assert_eq!(format!("{}", HsmKeyAlgorithm::Rsa4096), "RSA-4096");
}

#[test]
fn display_ecdsa_p256() {
    assert_eq!(format!("{}", HsmKeyAlgorithm::EcdsaP256), "ECDSA-P256");
}

#[test]
fn display_ecdsa_p384() {
    assert_eq!(format!("{}", HsmKeyAlgorithm::EcdsaP384), "ECDSA-P384");
}

#[test]
fn display_ed25519() {
    assert_eq!(format!("{}", HsmKeyAlgorithm::Ed25519), "ED25519");
}
