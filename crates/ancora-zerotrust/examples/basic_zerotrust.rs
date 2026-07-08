use ancora_zerotrust::audit::{ZtAction, ZtAuditEntry, ZtAuditLog};
use ancora_zerotrust::builder::{IdentityBuilder, SessionBuilder};
use ancora_zerotrust::device::{DevicePosture, DeviceStore};
use ancora_zerotrust::evaluator::ZeroTrustEvaluator;
use ancora_zerotrust::identity::IdentityKind;
use ancora_zerotrust::presets::strict_policy;
use ancora_zerotrust::request::AccessRequest;
use ancora_zerotrust::session::SessionStore;
use ancora_zerotrust::summary::ZeroTrustSummary;

fn main() {
    let mut devices = DeviceStore::new();
    let mut sessions = SessionStore::new();
    let mut audit = ZtAuditLog::new();

    let identity = IdentityBuilder::new("alice", "acme", IdentityKind::Human)
        .tick(1000)
        .group("admin")
        .build();

    let mut device = DevicePosture::new("laptop-alice", "acme", 1000);
    device.os_up_to_date = true;
    device.antivirus_active = true;
    device.disk_encrypted = true;
    device.compute_trust();
    devices.upsert(device);

    let session = SessionBuilder::new("sess-1", "acme", "alice")
        .created_at(1000)
        .expires_at(5000)
        .device("laptop-alice")
        .build();
    sessions.insert(session);

    let policy = strict_policy("acme");
    let request = AccessRequest::new("req-1", "acme", "alice", "admin/dashboard", "READ", 1000)
        .with_device("laptop-alice");

    let decision = ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices);
    println!("Decision: {:?}", decision);

    audit.record(ZtAuditEntry::new(
        1000,
        "acme",
        "alice",
        "admin/dashboard",
        ZtAction::PolicyEvaluated,
        true,
        "evaluated",
    ));

    let identities = vec![&identity];
    let summary = ZeroTrustSummary::generate(&identities, &sessions, &audit, "acme", 1000);
    println!(
        "Active sessions: {} Denied: {} Healthy: {}",
        summary.active_sessions, summary.denied_requests, summary.is_healthy
    );
}
