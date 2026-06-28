use crate::{ExpiryChecker, SecretKind, SecretStore, Secret};
#[test]
fn secret_with_ttl_reports_expired_after_deadline() {
    let mut store = SecretStore::new();
    let mut s = Secret::new("api/key", "t1", SecretKind::ApiKey, "val", 100);
    s.ttl_ticks = Some(50);
    store.create("t1", "api/key", SecretKind::ApiKey, "val", 100).unwrap();
    let secret = store.read_mut("t1", "api/key").unwrap();
    secret.ttl_ticks = Some(50);
    let expired = ExpiryChecker::is_expired(&store, "t1", "api/key", 200).unwrap();
    assert!(expired);
}
