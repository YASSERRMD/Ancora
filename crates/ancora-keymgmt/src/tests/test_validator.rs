use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore, KeyValidator, ValidationIssue};
#[test]
fn valid_key_has_no_issues() {
    let k = CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "material");
    assert!(KeyValidator::is_valid_key(&k, 0));
}
#[test]
fn destroyed_key_with_material_is_invalid() {
    let mut k = CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "material");
    // Force destroyed without clearing material to simulate the issue
    k.status = crate::KeyStatus::Destroyed;
    let issues = KeyValidator::validate_key(&k, 0);
    assert!(issues.contains(&ValidationIssue::DestroyedKeyHasMaterial("k1".to_string())));
}
#[test]
fn tenant_with_no_active_keys_fails_validation() {
    let store = KeyStore::new();
    let issues = KeyValidator::validate_tenant(&store, "t1", 0);
    assert!(issues.contains(&ValidationIssue::NoActiveKeysForTenant("t1".to_string())));
}
