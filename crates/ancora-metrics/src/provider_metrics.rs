use std::collections::HashMap;

/// Per-provider error rate tracker.
#[derive(Default)]
pub struct ProviderErrorRate {
    success: HashMap<String, u64>,
    errors: HashMap<String, u64>,
}

impl ProviderErrorRate {
    pub fn record_success(&mut self, provider: &str) {
        *self.success.entry(provider.into()).or_default() += 1;
    }

    pub fn record_error(&mut self, provider: &str) {
        *self.errors.entry(provider.into()).or_default() += 1;
    }

    pub fn error_rate(&self, provider: &str) -> f64 {
        let s = self.success.get(provider).copied().unwrap_or(0);
        let e = self.errors.get(provider).copied().unwrap_or(0);
        let total = s + e;
        if total == 0 {
            0.0
        } else {
            e as f64 / total as f64
        }
    }
}

/// Cost rate per tenant (tokens * cost_per_token summed over a window).
#[derive(Default)]
pub struct TenantCostRate {
    totals: HashMap<String, f64>,
}

impl TenantCostRate {
    pub fn add(&mut self, tenant: &str, cost: f64) {
        *self.totals.entry(tenant.into()).or_default() += cost;
    }

    pub fn total(&self, tenant: &str) -> f64 {
        self.totals.get(tenant).copied().unwrap_or(0.0)
    }
}

/// Journal append latency histogram (shared instance).
pub fn journal_latency_buckets() -> Vec<u64> {
    vec![1, 5, 10, 25, 50, 100, 250, 500, 1000]
}
