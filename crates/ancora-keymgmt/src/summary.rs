use crate::expiry::ExpiryChecker;
use crate::stats::KeyStats;
use crate::store::KeyStore;
use crate::validator::KeyValidator;

pub struct TenantKeySummary {
    pub tenant_id: String,
    pub active_count: usize,
    pub expired_count: usize,
    pub expiring_soon_count: usize,
    pub validation_issue_count: usize,
    pub by_algorithm: std::collections::HashMap<String, usize>,
}

impl TenantKeySummary {
    pub fn generate(store: &KeyStore, tenant_id: &str, current_tick: u64, warning_ticks: u64) -> Self {
        let stats = KeyStats::for_tenant(store, tenant_id);
        let expired = ExpiryChecker::expired_keys(store, tenant_id, current_tick);
        let expiring_soon = ExpiryChecker::expiring_soon(store, tenant_id, current_tick, warning_ticks);
        let issues = KeyValidator::validate_tenant(store, tenant_id, current_tick);
        Self {
            tenant_id: tenant_id.to_string(),
            active_count: stats.total_active,
            expired_count: expired.len(),
            expiring_soon_count: expiring_soon.len(),
            validation_issue_count: issues.len(),
            by_algorithm: stats.by_algorithm,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.active_count > 0 && self.validation_issue_count == 0
    }
}
