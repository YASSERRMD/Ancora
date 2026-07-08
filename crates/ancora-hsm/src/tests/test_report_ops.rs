use crate::audit::{HsmAuditEntry, HsmAuditLog, HsmOperation};
use crate::key::HsmKeyAlgorithm;
use crate::mock::SoftHsm;
use crate::report::HsmReport;
use crate::session::SessionManager;
use crate::slot::SlotManager;

#[test]
fn report_audit_failures() {
    let hsm = SoftHsm::new();
    let slots = SlotManager::new();
    let sessions = SessionManager::new();
    let mut audit = HsmAuditLog::new();
    audit.record(HsmAuditEntry::new(
        1,
        0,
        HsmOperation::GenerateKey,
        true,
        "",
    ));
    audit.record(HsmAuditEntry::new(2, 0, HsmOperation::Sign, true, ""));
    audit.record(HsmAuditEntry::new(
        3,
        0,
        HsmOperation::Encrypt,
        false,
        "fail",
    ));
    let r = HsmReport::generate(&hsm, &slots, &sessions, &audit, 100);
    assert_eq!(r.audit_failures, 1);
}

#[test]
fn report_operations_from_hsm() {
    let mut hsm = SoftHsm::new();
    hsm.generate_key(0, "k1", HsmKeyAlgorithm::Aes256, 1);
    hsm.generate_key(0, "k2", HsmKeyAlgorithm::Aes256, 1);
    let slots = SlotManager::new();
    let sessions = SessionManager::new();
    let audit = HsmAuditLog::new();
    let r = HsmReport::generate(&hsm, &slots, &sessions, &audit, 100);
    assert_eq!(r.total_operations, 2);
    assert_eq!(r.total_keys, 2);
}

#[test]
fn report_tick_recorded() {
    let hsm = SoftHsm::new();
    let slots = SlotManager::new();
    let sessions = SessionManager::new();
    let audit = HsmAuditLog::new();
    let r = HsmReport::generate(&hsm, &slots, &sessions, &audit, 42);
    assert_eq!(r.tick, 42);
}
