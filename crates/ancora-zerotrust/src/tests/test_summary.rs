use crate::audit::ZtAuditLog;
use crate::identity::{Identity, IdentityKind};
use crate::session::SessionStore;
use crate::summary::ZeroTrustSummary;

#[test]
fn summary_healthy_no_denied() {
    let sessions = SessionStore::new();
    let audit = ZtAuditLog::new();
    let s = ZeroTrustSummary::generate(&[], &sessions, &audit, "t1", 100);
    assert!(s.is_healthy);
    assert_eq!(s.denied_requests, 0);
}

#[test]
fn summary_active_identities() {
    let i1 = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let mut i2 = Identity::new("i2", "t1", IdentityKind::Human, 1);
    i2.suspend();
    let v: Vec<&Identity> = vec![&i1, &i2];
    let sessions = SessionStore::new();
    let audit = ZtAuditLog::new();
    let s = ZeroTrustSummary::generate(&v, &sessions, &audit, "t1", 100);
    assert_eq!(s.active_identities, 1);
}
