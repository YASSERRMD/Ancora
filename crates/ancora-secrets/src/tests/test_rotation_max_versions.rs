use crate::{RotationPolicy, SecretKind, SecretStore};
#[test]
fn rotation_prunes_old_versions_beyond_max() {
    let mut store = SecretStore::new();
    store.create("t1", "k", SecretKind::Opaque, "v0", 0).unwrap();
    let policy = RotationPolicy::new(3);
    for i in 1u64..=5 {
        policy.rotate(&mut store, "t1", "k", format!("v{}", i), i).unwrap();
    }
    let count = policy.versions_retained(&store, "t1", "k").unwrap();
    assert!(count <= 3);
}
