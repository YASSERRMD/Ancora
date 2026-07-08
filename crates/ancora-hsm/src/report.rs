use crate::audit::HsmAuditLog;
use crate::mock::SoftHsm;
use crate::session::SessionManager;
use crate::slot::SlotManager;

pub struct HsmReport {
    pub total_slots: usize,
    pub slots_with_token: usize,
    pub total_keys: usize,
    pub active_sessions: usize,
    pub total_operations: usize,
    pub audit_failures: usize,
    pub tick: u64,
}

impl HsmReport {
    pub fn generate(
        hsm: &SoftHsm,
        slots: &SlotManager,
        sessions: &SessionManager,
        audit: &HsmAuditLog,
        tick: u64,
    ) -> Self {
        Self {
            total_slots: slots.count(),
            slots_with_token: slots.slots_with_token().len(),
            total_keys: hsm.key_count(),
            active_sessions: sessions.active().len(),
            total_operations: hsm.operation_count(),
            audit_failures: audit.failures().len(),
            tick,
        }
    }
}
