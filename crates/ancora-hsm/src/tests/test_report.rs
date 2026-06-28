use crate::audit::HsmAuditLog;
use crate::key::HsmKeyAlgorithm;
use crate::mock::SoftHsm;
use crate::report::HsmReport;
use crate::session::SessionManager;
use crate::slot::{HsmSlot, SlotManager};

#[test]
fn report_empty() {
    let hsm = SoftHsm::new();
    let slots = SlotManager::new();
    let sessions = SessionManager::new();
    let audit = HsmAuditLog::new();
    let r = HsmReport::generate(&hsm, &slots, &sessions, &audit, 100);
    assert_eq!(r.total_slots, 0);
    assert_eq!(r.total_keys, 0);
}

#[test]
fn report_with_data() {
    let mut hsm = SoftHsm::new();
    hsm.generate_key(0, "k", HsmKeyAlgorithm::Aes256, 1);
    let mut slots = SlotManager::new();
    let mut slot = HsmSlot::new(0, "S0", "SoftHSM");
    slot.insert_token();
    slots.add_slot(slot);
    let sessions = SessionManager::new();
    let audit = HsmAuditLog::new();
    let r = HsmReport::generate(&hsm, &slots, &sessions, &audit, 100);
    assert_eq!(r.total_slots, 1);
    assert_eq!(r.slots_with_token, 1);
    assert_eq!(r.total_keys, 1);
}
