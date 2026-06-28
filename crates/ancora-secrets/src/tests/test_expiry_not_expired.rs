use crate::{ExpiryChecker, SecretKind, SecretStore};
#[test]
fn secret_without_ttl_is_not_expired() {
    let mut store = SecretStore::new();
    store.create("t1", "db/pass", SecretKind::Opaque, "val", 1).unwrap();
    let expired = ExpiryChecker::is_expired(&store, "t1", "db/pass", 99999).unwrap();
    assert!(!expired);
}
