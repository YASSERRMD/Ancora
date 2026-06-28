use crate::key::HsmKeyAlgorithm;
use crate::policy::HsmPolicy;

#[test]
fn no_algorithms_blocked_by_default() {
    let p = HsmPolicy::new();
    assert!(p.algorithm_allowed(&HsmKeyAlgorithm::Rsa2048));
    assert!(p.algorithm_allowed(&HsmKeyAlgorithm::Aes128));
}

#[test]
fn block_multiple_algorithms() {
    let p = HsmPolicy::new()
        .block_algorithm(HsmKeyAlgorithm::Rsa2048)
        .block_algorithm(HsmKeyAlgorithm::Aes128);
    assert!(!p.algorithm_allowed(&HsmKeyAlgorithm::Rsa2048));
    assert!(!p.algorithm_allowed(&HsmKeyAlgorithm::Aes128));
    assert!(p.algorithm_allowed(&HsmKeyAlgorithm::Aes256));
}

#[test]
fn block_does_not_affect_others() {
    let p = HsmPolicy::new().block_algorithm(HsmKeyAlgorithm::EcdsaP256);
    assert!(p.algorithm_allowed(&HsmKeyAlgorithm::EcdsaP384));
    assert!(p.algorithm_allowed(&HsmKeyAlgorithm::Ed25519));
}
