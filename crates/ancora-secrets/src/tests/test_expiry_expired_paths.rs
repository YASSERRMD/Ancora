use crate::{ExpiryChecker, SecretKind, SecretStore};
#[test]
fn expired_paths_lists_only_expired() {
    let mut store = SecretStore::new();
    store
        .create("t1", "db/pass", SecretKind::Opaque, "v", 1)
        .unwrap();
    store
        .create("t1", "api/key", SecretKind::ApiKey, "k", 1)
        .unwrap();
    let secret = store.read_mut("t1", "api/key").unwrap();
    secret.ttl_ticks = Some(10);
    let expired = ExpiryChecker::expired_paths(&store, "t1", 20);
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0], "api/key");
}
