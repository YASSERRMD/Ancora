use crate::key::HsmKeyAlgorithm;
use crate::mock::SoftHsm;

#[test]
fn generate_and_get_key() {
    let mut hsm = SoftHsm::new();
    let handle = hsm.generate_key(0, "test", HsmKeyAlgorithm::Aes256, 1);
    assert!(hsm.get_key(handle).is_some());
    assert_eq!(hsm.key_count(), 1);
}

#[test]
fn delete_key() {
    let mut hsm = SoftHsm::new();
    let h = hsm.generate_key(0, "k", HsmKeyAlgorithm::Ed25519, 1);
    assert!(hsm.delete_key(h));
    assert_eq!(hsm.key_count(), 0);
    assert!(!hsm.delete_key(h));
}

#[test]
fn keys_for_slot() {
    let mut hsm = SoftHsm::new();
    hsm.generate_key(0, "k1", HsmKeyAlgorithm::Aes256, 1);
    hsm.generate_key(1, "k2", HsmKeyAlgorithm::Aes256, 1);
    assert_eq!(hsm.keys_for_slot(0).len(), 1);
    assert_eq!(hsm.keys_for_slot(1).len(), 1);
}

#[test]
fn operation_count() {
    let mut hsm = SoftHsm::new();
    hsm.generate_key(0, "k", HsmKeyAlgorithm::Aes128, 1);
    assert_eq!(hsm.operation_count(), 1);
}
