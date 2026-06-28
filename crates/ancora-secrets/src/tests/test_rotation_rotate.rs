use crate::{RotationPolicy, SecretKind, SecretStore};
#[test]
fn rotation_creates_new_version() {
    let mut store = SecretStore::new();
    store.create("t1", "db/pass", SecretKind::Opaque, "v1", 1).unwrap();
    let policy = RotationPolicy::default_policy();
    let new_ver = policy.rotate(&mut store, "t1", "db/pass", "v2", 2).unwrap();
    assert_eq!(new_ver, 2);
    let s = store.read("t1", "db/pass").unwrap();
    assert_eq!(s.active_value(), Some("v2"));
}
