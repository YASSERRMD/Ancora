use crate::{SecretKind, SecretStore};
#[test]
fn delete_removes_secret() {
    let mut store = SecretStore::new();
    store.create("t1", "tmp/key", SecretKind::Opaque, "val", 1).unwrap();
    store.delete("t1", "tmp/key").unwrap();
    assert!(store.read("t1", "tmp/key").is_err());
}
#[test]
fn delete_missing_returns_error() {
    let mut store = SecretStore::new();
    assert!(store.delete("t1", "nope/path").is_err());
}
