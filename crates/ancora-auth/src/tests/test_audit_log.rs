use crate::{AuthAuditLog, AuthEvent};

#[test]
fn audit_log_records_events() {
    let mut log = AuthAuditLog::new(100);
    log.record(AuthEvent::OidcLoginSuccess {
        subject: "alice".into(),
        tenant_id: "t1".into(),
        tick: 10,
    });
    assert_eq!(log.count(), 1);
}

#[test]
fn audit_log_capped_at_max_size() {
    let mut log = AuthAuditLog::new(3);
    for i in 0..5u64 {
        log.record(AuthEvent::TokenRevoked {
            token_prefix: format!("tok-{i}"),
            tick: i,
        });
    }
    assert_eq!(log.count(), 3);
}

#[test]
fn audit_log_failures_for_tenant() {
    let mut log = AuthAuditLog::new(100);
    log.record(AuthEvent::OidcLoginFailure {
        tenant_id: "t1".into(),
        reason: "bad code".into(),
        tick: 1,
    });
    log.record(AuthEvent::OidcLoginFailure {
        tenant_id: "t2".into(),
        reason: "expired".into(),
        tick: 2,
    });
    let t1_failures = log.failures_for_tenant("t1");
    assert_eq!(t1_failures.len(), 1);
}
