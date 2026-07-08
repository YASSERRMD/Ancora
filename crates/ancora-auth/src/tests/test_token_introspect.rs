use crate::{IntrospectStatus, Token, TokenIntrospector, TokenKind};

fn make_token(raw: &str, subject: &str, tenant: &str, exp: u64) -> Token {
    Token::new(
        raw,
        TokenKind::Bearer,
        subject,
        tenant,
        exp,
        vec!["openid".into()],
    )
}

#[test]
fn introspect_active_token() {
    let mut intr = TokenIntrospector::new();
    let token = make_token("tok-1", "alice", "tenant-a", 500);
    intr.register(token);
    let result = intr.introspect("tok-1", 100);
    assert_eq!(result.status, IntrospectStatus::Active);
    assert!(result.is_active());
}

#[test]
fn introspect_expired_token() {
    let mut intr = TokenIntrospector::new();
    let token = make_token("tok-2", "alice", "tenant-a", 50);
    intr.register(token);
    let result = intr.introspect("tok-2", 100);
    assert_eq!(result.status, IntrospectStatus::Expired);
}

#[test]
fn introspect_unknown_token() {
    let intr = TokenIntrospector::new();
    let result = intr.introspect("no-such-token", 100);
    assert_eq!(result.status, IntrospectStatus::Unknown);
    assert!(!result.is_active());
}

#[test]
fn introspect_revoked_token() {
    let mut intr = TokenIntrospector::new();
    let mut token = make_token("tok-3", "alice", "tenant-a", 500);
    token.revoked = true;
    intr.register(token);
    let result = intr.introspect("tok-3", 100);
    assert_eq!(result.status, IntrospectStatus::Revoked);
}
