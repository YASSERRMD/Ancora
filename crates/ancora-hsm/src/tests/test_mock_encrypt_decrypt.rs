use crate::key::HsmKeyAlgorithm;
use crate::mock::SoftHsm;

#[test]
fn encrypt_decrypt_roundtrip() {
    let mut hsm = SoftHsm::new();
    let h = hsm.generate_key(0, "enc", HsmKeyAlgorithm::Aes256, 1);
    let data = b"secret data";
    let encrypted = hsm.encrypt(h, data).expect("encrypt");
    let decrypted = hsm.decrypt(h, &encrypted).expect("decrypt");
    assert_eq!(&decrypted, data);
}

#[test]
fn encrypt_missing_key_returns_none() {
    let hsm = SoftHsm::new();
    assert!(hsm.encrypt(999, b"x").is_none());
}

#[test]
fn decrypt_missing_key_returns_none() {
    let hsm = SoftHsm::new();
    assert!(hsm.decrypt(999, b"x").is_none());
}

#[test]
fn encrypt_transforms_bytes() {
    let mut hsm = SoftHsm::new();
    let h = hsm.generate_key(0, "k", HsmKeyAlgorithm::Aes128, 1);
    let plain = vec![0u8, 100u8, 255u8];
    let enc = hsm.encrypt(h, &plain).unwrap();
    assert_ne!(enc, plain);
    assert_eq!(enc.len(), plain.len());
}
