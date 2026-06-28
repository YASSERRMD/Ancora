use crate::{revoke_all, RevocationStore};

#[test]
fn bulk_revoke_empty_iter_is_noop() {
    let mut store = RevocationStore::new();
    revoke_all(&mut store, vec![]);
    assert_eq!(store.count(), 0);
}

#[test]
fn bulk_revoke_all_present() {
    let mut store = RevocationStore::new();
    let tokens: Vec<String> = (0..10).map(|i| format!("tok-{i}")).collect();
    revoke_all(&mut store, tokens.clone());
    assert_eq!(store.count(), 10);
    for t in &tokens {
        assert!(store.is_revoked(t));
    }
}

#[test]
fn bulk_revoke_idempotent() {
    let mut store = RevocationStore::new();
    let tokens = vec!["a".to_string(), "b".to_string()];
    revoke_all(&mut store, tokens.clone());
    revoke_all(&mut store, tokens);
    assert_eq!(store.count(), 2);
}
