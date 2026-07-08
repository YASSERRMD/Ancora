use crate::session::{SessionStore, ZeroTrustSession};

#[test]
fn session_store_for_identity() {
    let mut store = SessionStore::new();
    store.insert(ZeroTrustSession::new("s1", "t1", "i1", 0, 9999));
    store.insert(ZeroTrustSession::new("s2", "t1", "i2", 0, 9999));
    assert_eq!(store.for_identity("i1").len(), 1);
    assert_eq!(store.for_identity("i2").len(), 1);
}

#[test]
fn session_store_count() {
    let mut store = SessionStore::new();
    for i in 0..4 {
        store.insert(ZeroTrustSession::new(
            format!("s{}", i),
            "t1",
            "i1",
            0,
            9999,
        ));
    }
    assert_eq!(store.count(), 4);
}
