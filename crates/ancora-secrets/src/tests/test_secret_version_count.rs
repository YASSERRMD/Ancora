use crate::{Secret, SecretKind, SecretStore};
#[test]
fn write_version_increments_version_count() {
    let mut store = SecretStore::new();
    store.create("t1", "db/pass", SecretKind::Opaque, "v1", 1).unwrap();
    store.write_version("t1", "db/pass", "v2", 2).unwrap();
    let s = store.read("t1", "db/pass").unwrap();
    assert_eq!(s.version_count(), 2);
    assert_eq!(s.active_version, 2);
}
