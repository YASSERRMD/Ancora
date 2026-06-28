use crate::audit::AirGapAuditLog;
use crate::boundary::AirGapBoundary;
use crate::store::TransferStore;

pub struct AirGapReport {
    pub total_zones: usize,
    pub restricted_zones: usize,
    pub total_transfers: usize,
    pub pending_transfers: usize,
    pub total_audit_entries: usize,
    pub tick: u64,
}

impl AirGapReport {
    pub fn generate(
        boundary: &AirGapBoundary,
        store: &TransferStore,
        audit: &AirGapAuditLog,
        tick: u64,
    ) -> Self {
        Self {
            total_zones: boundary.count(),
            restricted_zones: boundary.restricted_zones().len(),
            total_transfers: store.count(),
            pending_transfers: store.pending().len(),
            total_audit_entries: audit.count(),
            tick,
        }
    }
}
