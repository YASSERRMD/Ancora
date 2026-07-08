use crate::key::HsmKeyAlgorithm;
use crate::mock::SoftHsm;
use crate::presets::{aes256_key, default_slot, ed25519_signing_key, strict_hsm_policy};

#[test]
fn aes256_preset() {
    let mut hsm = SoftHsm::new();
    let h = aes256_key(&mut hsm, 0, 1);
    assert!(hsm.get_key(h).is_some());
    assert_eq!(
        hsm.get_key(h).map(|k| k.algorithm.clone()),
        Some(HsmKeyAlgorithm::Aes256)
    );
}

#[test]
fn ed25519_preset() {
    let mut hsm = SoftHsm::new();
    let h = ed25519_signing_key(&mut hsm, 0, 1);
    assert!(hsm.get_key(h).is_some());
}

#[test]
fn default_slot_preset() {
    let s = default_slot();
    assert!(s.has_token());
}

#[test]
fn strict_policy_preset() {
    let p = strict_hsm_policy();
    assert!(!p.algorithm_allowed(&HsmKeyAlgorithm::Rsa2048));
    assert!(p.algorithm_allowed(&HsmKeyAlgorithm::Aes256));
}
