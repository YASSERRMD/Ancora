use crate::{JwkKey, JwksStore};

#[test]
fn no_active_keys_when_all_expired() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("k1", "m", "AQAB", 0, 50));
    store.add_key(JwkKey::new("k2", "m", "AQAB", 0, 30));
    let active = store.active_keys(100);
    assert!(active.is_empty());
}

#[test]
fn multiple_overlapping_keys_active() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("k1", "m1", "AQAB", 0, 500));
    store.add_key(JwkKey::new("k2", "m2", "AQAB", 100, 500));
    let active = store.active_keys(200);
    assert_eq!(active.len(), 2);
}

#[test]
fn key_not_yet_active() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("future", "m", "AQAB", 500, 999));
    let active = store.active_keys(100);
    assert!(active.is_empty());
}
