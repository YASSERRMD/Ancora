/// Snapshot of current quota consumption for a tenant.
#[derive(Debug, Clone)]
pub struct QuotaUsage {
    pub tenant: String,
    pub requests: u64,
    pub max_requests: u64,
    pub tokens: u64,
    pub max_tokens: u64,
    pub cost_usd: f64,
    pub max_cost_usd: f64,
}

impl QuotaUsage {
    pub fn request_pct(&self) -> f64 {
        if self.max_requests == 0 { return 0.0; }
        (self.requests as f64 / self.max_requests as f64) * 100.0
    }

    pub fn token_pct(&self) -> f64 {
        if self.max_tokens == 0 { return 0.0; }
        (self.tokens as f64 / self.max_tokens as f64) * 100.0
    }

    pub fn cost_pct(&self) -> f64 {
        if self.max_cost_usd == 0.0 { return 0.0; }
        (self.cost_usd / self.max_cost_usd) * 100.0
    }
}
