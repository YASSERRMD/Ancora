use crate::{JwkKey, JwksStore};

#[test]
fn jwks_rotation_removes_old_key() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("old", "mod-old", "AQAB", 0, 100));
    assert!(store.get_key("old").is_some());
    let new_key = JwkKey::new("new", "mod-new", "AQAB", 100, 999);
    store.rotate("old", new_key);
    assert!(store.get_key("old").is_none());
    assert!(store.get_key("new").is_some());
}

#[test]
fn jwks_active_keys_returns_only_current() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("past", "m1", "AQAB", 0, 50));
    store.add_key(JwkKey::new("current", "m2", "AQAB", 50, 200));
    store.add_key(JwkKey::new("future", "m3", "AQAB", 200, 999));
    let active = store.active_keys(100);
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].kid, "current");
}

#[test]
fn jwks_key_count_correct() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("k1", "m1", "AQAB", 0, 100));
    store.add_key(JwkKey::new("k2", "m2", "AQAB", 0, 100));
    assert_eq!(store.key_count(), 2);
}
