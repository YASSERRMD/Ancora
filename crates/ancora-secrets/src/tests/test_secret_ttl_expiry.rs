use crate::{Secret, SecretKind};
#[test]
fn secret_with_ttl_expires_after_deadline() {
    let s = Secret::new("api/key", "t1", SecretKind::ApiKey, "val", 100).with_ttl(50);
    assert!(!s.is_expired(140));
    assert!(s.is_expired(151));
}
#[test]
fn secret_without_ttl_never_expires() {
    let s = Secret::new("api/key", "t1", SecretKind::ApiKey, "val", 1);
    assert!(!s.is_expired(u64::MAX));
}
