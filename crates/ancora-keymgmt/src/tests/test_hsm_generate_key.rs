use crate::{HsmConfig, HsmStub, KeyAlgorithm, KeyPurpose, KeyStatus};
#[test]
fn hsm_generate_key_returns_active_key() {
    let mut hsm = HsmStub::new(HsmConfig::software());
    let k = hsm.generate_key("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 10);
    assert_eq!(k.status, KeyStatus::Active);
    assert_eq!(k.algorithm, KeyAlgorithm::Aes256);
    assert!(k.key_material.starts_with("hsm-"));
}
