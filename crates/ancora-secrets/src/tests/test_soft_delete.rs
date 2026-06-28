use crate::{is_soft_deleted, soft_delete, SecretKind, SecretStore};
#[test]
fn soft_delete_marks_active_version_deleted() {
    let mut store = SecretStore::new();
    store.create("t1", "tmp/key", SecretKind::Opaque, "val", 1).unwrap();
    soft_delete(&mut store, "t1", "tmp/key").unwrap();
    assert!(is_soft_deleted(&store, "t1", "tmp/key").unwrap());
}
#[test]
fn non_deleted_secret_is_not_soft_deleted() {
    let mut store = SecretStore::new();
    store.create("t1", "active/key", SecretKind::Opaque, "val", 1).unwrap();
    assert!(!is_soft_deleted(&store, "t1", "active/key").unwrap());
}
#[test]
fn soft_delete_secret_has_no_active_value() {
    let mut store = SecretStore::new();
    store.create("t1", "k", SecretKind::Opaque, "v", 1).unwrap();
    soft_delete(&mut store, "t1", "k").unwrap();
    let s = store.read("t1", "k").unwrap();
    assert!(s.active_value().is_none());
}
