use crate::identity::{Identity, IdentityKind};
use crate::session::SessionStore;
use crate::audit::{ZtAction, ZtAuditEntry, ZtAuditLog};
use crate::summary::ZeroTrustSummary;

#[test]
fn unhealthy_with_denied_requests() {
    let sessions = SessionStore::new();
    let mut audit = ZtAuditLog::new();
    audit.record(ZtAuditEntry::new(1, "t1", "i1", "res", ZtAction::AccessDenied, false, "blocked"));
    let s = ZeroTrustSummary::generate(&[], &sessions, &audit, "t1", 100);
    assert!(!s.is_healthy);
    assert_eq!(s.denied_requests, 1);
}
