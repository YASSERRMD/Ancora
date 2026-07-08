use crate::audit::ZtAuditLog;
use crate::identity::Identity;
use crate::session::SessionStore;

pub struct ZeroTrustSummary {
    pub tenant_id: String,
    pub active_identities: usize,
    pub active_sessions: usize,
    pub denied_requests: usize,
    pub is_healthy: bool,
}

impl ZeroTrustSummary {
    pub fn generate(
        identities: &[&Identity],
        sessions: &SessionStore,
        audit: &ZtAuditLog,
        tenant_id: &str,
        tick: u64,
    ) -> Self {
        let active_identities = identities
            .iter()
            .filter(|i| i.tenant_id == tenant_id && i.is_active())
            .count();
        let active_sessions = sessions
            .active(tick)
            .iter()
            .filter(|s| s.tenant_id == tenant_id)
            .count();
        let denied_requests = audit
            .denied()
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .count();
        let is_healthy = denied_requests == 0;
        Self {
            tenant_id: tenant_id.to_string(),
            active_identities,
            active_sessions,
            denied_requests,
            is_healthy,
        }
    }
}
