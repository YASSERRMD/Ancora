use crate::{RotationPolicy, SecretKind, SecretStore};
#[test]
fn versions_retained_returns_current_count() {
    let mut store = SecretStore::new();
    store
        .create("t1", "k", SecretKind::Opaque, "v0", 0)
        .unwrap();
    store.write_version("t1", "k", "v1", 1).unwrap();
    let policy = RotationPolicy::default_policy();
    let count = policy.versions_retained(&store, "t1", "k").unwrap();
    assert_eq!(count, 2);
}
