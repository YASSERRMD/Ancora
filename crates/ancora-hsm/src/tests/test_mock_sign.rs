use crate::key::HsmKeyAlgorithm;
use crate::mock::SoftHsm;

#[test]
fn sign_returns_some() {
    let mut hsm = SoftHsm::new();
    let h = hsm.generate_key(0, "signing", HsmKeyAlgorithm::Ed25519, 1);
    let sig = hsm.sign(h, b"hello world");
    assert!(sig.is_some());
}

#[test]
fn sign_missing_key_returns_none() {
    let hsm = SoftHsm::new();
    let sig = hsm.sign(999, b"data");
    assert!(sig.is_none());
}

#[test]
fn sign_appends_handle_bytes() {
    let mut hsm = SoftHsm::new();
    let h = hsm.generate_key(0, "k", HsmKeyAlgorithm::EcdsaP256, 1);
    let msg = b"msg";
    let sig = hsm.sign(h, msg).unwrap();
    assert!(sig.len() > msg.len());
    assert_eq!(&sig[..msg.len()], msg);
}
