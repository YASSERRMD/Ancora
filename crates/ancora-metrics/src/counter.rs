use std::collections::HashMap;

/// Run success and failure counters, scoped by tenant.
#[derive(Default)]
pub struct RunCounters {
    success: HashMap<String, u64>,
    failure: HashMap<String, u64>,
}

impl RunCounters {
    pub fn record_success(&mut self, tenant: &str) {
        *self.success.entry(tenant.into()).or_default() += 1;
    }

    pub fn record_failure(&mut self, tenant: &str) {
        *self.failure.entry(tenant.into()).or_default() += 1;
    }

    pub fn success_total(&self, tenant: &str) -> u64 {
        self.success.get(tenant).copied().unwrap_or(0)
    }

    pub fn failure_total(&self, tenant: &str) -> u64 {
        self.failure.get(tenant).copied().unwrap_or(0)
    }

    pub fn total(&self, tenant: &str) -> u64 {
        self.success_total(tenant) + self.failure_total(tenant)
    }

    pub fn error_rate(&self, tenant: &str) -> f64 {
        let total = self.total(tenant);
        if total == 0 {
            return 0.0;
        }
        self.failure_total(tenant) as f64 / total as f64
    }
}
