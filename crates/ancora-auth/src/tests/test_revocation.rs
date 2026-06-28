use crate::{revoke_all, RevocationStore};

#[test]
fn revoke_single_token() {
    let mut store = RevocationStore::new();
    store.revoke("tok-1");
    assert!(store.is_revoked("tok-1"));
    assert!(!store.is_revoked("tok-2"));
}

#[test]
fn bulk_revoke_all_tokens() {
    let mut store = RevocationStore::new();
    let tokens = vec!["t1".to_string(), "t2".to_string(), "t3".to_string()];
    revoke_all(&mut store, tokens);
    assert_eq!(store.count(), 3);
    assert!(store.is_revoked("t1"));
    assert!(store.is_revoked("t2"));
    assert!(store.is_revoked("t3"));
}

#[test]
fn non_revoked_token_not_found() {
    let store = RevocationStore::new();
    assert!(!store.is_revoked("tok-unknown"));
}
