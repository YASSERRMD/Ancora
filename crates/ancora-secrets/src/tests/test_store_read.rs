use crate::{SecretKind, SecretStore};
#[test]
fn read_existing_secret_returns_it() {
    let mut store = SecretStore::new();
    store
        .create("t1", "db/pass", SecretKind::Opaque, "secret", 1)
        .unwrap();
    let s = store.read("t1", "db/pass").unwrap();
    assert_eq!(s.active_value(), Some("secret"));
}
#[test]
fn read_missing_secret_returns_error() {
    let store = SecretStore::new();
    assert!(store.read("t1", "missing/path").is_err());
}
