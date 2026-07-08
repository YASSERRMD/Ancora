use crate::audit::ZtAuditLog;
use crate::device::DeviceStore;
use crate::identity::Identity;
use crate::session::SessionStore;

pub struct ZeroTrustReport {
    pub tenant_id: String,
    pub total_identities: usize,
    pub active_sessions: usize,
    pub trusted_devices: usize,
    pub denied_requests: usize,
    pub audit_entries: usize,
    pub tick: u64,
}

impl ZeroTrustReport {
    pub fn generate(
        identities: &[&Identity],
        sessions: &SessionStore,
        devices: &DeviceStore,
        audit: &ZtAuditLog,
        tenant_id: &str,
        tick: u64,
    ) -> Self {
        let total_identities = identities
            .iter()
            .filter(|i| i.tenant_id == tenant_id)
            .count();
        Self {
            tenant_id: tenant_id.to_string(),
            total_identities,
            active_sessions: sessions
                .active(tick)
                .iter()
                .filter(|s| s.tenant_id == tenant_id)
                .count(),
            trusted_devices: devices
                .for_tenant(tenant_id)
                .iter()
                .filter(|d| d.is_trusted())
                .count(),
            denied_requests: audit
                .denied()
                .iter()
                .filter(|e| e.tenant_id == tenant_id)
                .count(),
            audit_entries: audit.for_tenant(tenant_id).len(),
            tick,
        }
    }
}
