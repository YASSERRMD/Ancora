use crate::{Token, TokenKind};

#[test]
fn token_not_expired_before_deadline() {
    let token = Token::new("t1", TokenKind::Bearer, "u", "ten", 500, vec![]);
    assert!(!token.is_expired(100));
    assert!(token.is_valid(100));
}

#[test]
fn token_expired_at_deadline() {
    let token = Token::new("t2", TokenKind::Bearer, "u", "ten", 100, vec![]);
    assert!(token.is_expired(100));
    assert!(!token.is_valid(100));
}

#[test]
fn revoked_token_invalid_even_if_not_expired() {
    let mut token = Token::new("t3", TokenKind::Bearer, "u", "ten", 999, vec![]);
    token.revoked = true;
    assert!(!token.is_expired(100));
    assert!(!token.is_valid(100));
}

#[test]
fn token_scope_check() {
    let token = Token::new("t4", TokenKind::Bearer, "u", "ten", 999, vec!["read:agents".into(), "write:tasks".into()]);
    assert!(token.has_scope("read:agents"));
    assert!(!token.has_scope("admin"));
}
