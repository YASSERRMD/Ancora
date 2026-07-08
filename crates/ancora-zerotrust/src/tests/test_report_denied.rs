use crate::audit::{ZtAction, ZtAuditEntry, ZtAuditLog};
use crate::device::DeviceStore;
use crate::identity::{Identity, IdentityKind};
use crate::report::ZeroTrustReport;
use crate::session::SessionStore;

#[test]
fn report_denied_requests() {
    let i1 = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let v: Vec<&Identity> = vec![&i1];
    let sessions = SessionStore::new();
    let devices = DeviceStore::new();
    let mut audit = ZtAuditLog::new();
    audit.record(ZtAuditEntry::new(
        1,
        "t1",
        "i1",
        "res",
        ZtAction::AccessDenied,
        false,
        "blocked",
    ));
    let r = ZeroTrustReport::generate(&v, &sessions, &devices, &audit, "t1", 100);
    assert_eq!(r.denied_requests, 1);
    assert_eq!(r.total_identities, 1);
}
