use crate::{IntrospectStatus, RevocationStore, Token, TokenIntrospector, TokenKind};

#[test]
fn revoked_token_introspects_as_revoked() {
    let mut intr = TokenIntrospector::new();
    let mut token = Token::new("revoked-tok", TokenKind::Bearer, "grace", "t", 999, vec![]);
    token.revoked = true;
    intr.register(token);
    let result = intr.introspect("revoked-tok", 100);
    assert_eq!(result.status, IntrospectStatus::Revoked);
    assert!(!result.is_active());
}

#[test]
fn revocation_store_correctly_flags_token() {
    let mut store = RevocationStore::new();
    assert!(!store.is_revoked("tok-x"));
    store.revoke("tok-x");
    assert!(store.is_revoked("tok-x"));
}

#[test]
fn double_revoke_idempotent() {
    let mut store = RevocationStore::new();
    store.revoke("tok-y");
    store.revoke("tok-y");
    assert_eq!(store.count(), 1);
}
