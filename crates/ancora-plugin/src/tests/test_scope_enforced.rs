//! Tests: plugin permission scoping is enforced.

use crate::permission::{
    enforce_required_scopes, require_scope, PermissionError, Scope, ScopeGrant,
};

fn make_grant(plugin_id: &str, scopes: &[&str]) -> ScopeGrant {
    let mut g = ScopeGrant::new(plugin_id);
    for s in scopes {
        g.grant(Scope::new(*s));
    }
    g
}

#[test]
fn required_scopes_all_granted_passes() {
    let grant = make_grant("my-plugin", &["llm:generate", "memory:read"]);
    let required = vec!["llm:generate".to_string(), "memory:read".to_string()];
    assert!(enforce_required_scopes("my-plugin", &required, &grant).is_ok());
}

#[test]
fn missing_scope_returns_error() {
    let grant = make_grant("my-plugin", &["llm:generate"]);
    let required = vec!["llm:generate".to_string(), "memory:write".to_string()];
    let err = enforce_required_scopes("my-plugin", &required, &grant).unwrap_err();
    assert!(matches!(err, PermissionError::MissingScopes { .. }));
}

#[test]
fn missing_scopes_error_lists_missing_names() {
    let grant = make_grant("my-plugin", &[]);
    let required = vec!["tool:execute".to_string()];
    let err = enforce_required_scopes("my-plugin", &required, &grant).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("tool:execute"), "error should name the missing scope: {msg}");
}

#[test]
fn require_scope_passes_when_granted() {
    let grant = make_grant("p", &["grader:run"]);
    assert!(require_scope("p", Scope::new("grader:run"), &grant).is_ok());
}

#[test]
fn require_scope_denied_when_not_granted() {
    let grant = make_grant("p", &[]);
    let err = require_scope("p", Scope::new("grader:run"), &grant).unwrap_err();
    assert!(matches!(err, PermissionError::Denied { .. }));
}

#[test]
fn revoke_removes_granted_scope() {
    let mut grant = make_grant("p", &["llm:generate"]);
    assert!(grant.has(&Scope::new("llm:generate")));
    grant.revoke(&Scope::new("llm:generate"));
    assert!(!grant.has(&Scope::new("llm:generate")));
}

#[test]
fn empty_required_scopes_always_passes() {
    let grant = make_grant("p", &[]);
    assert!(enforce_required_scopes("p", &[], &grant).is_ok());
}
