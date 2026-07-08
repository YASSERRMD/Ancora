use crate::audit::ZtAuditLog;
use crate::device::DeviceStore;
use crate::identity::{Identity, IdentityKind};
use crate::report::ZeroTrustReport;
use crate::session::SessionStore;

#[test]
fn report_empty() {
    let sessions = SessionStore::new();
    let devices = DeviceStore::new();
    let audit = ZtAuditLog::new();
    let r = ZeroTrustReport::generate(&[], &sessions, &devices, &audit, "t1", 100);
    assert_eq!(r.total_identities, 0);
    assert_eq!(r.active_sessions, 0);
    assert_eq!(r.denied_requests, 0);
}

#[test]
fn report_with_identities() {
    let i1 = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let i2 = Identity::new("i2", "t1", IdentityKind::Service, 1);
    let v: Vec<&Identity> = vec![&i1, &i2];
    let sessions = SessionStore::new();
    let devices = DeviceStore::new();
    let audit = ZtAuditLog::new();
    let r = ZeroTrustReport::generate(&v, &sessions, &devices, &audit, "t1", 100);
    assert_eq!(r.total_identities, 2);
}
