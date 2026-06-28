use serde::{Deserialize, Serialize};

/// Per-tenant quota limits.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuotaSchema {
    /// Max requests per window.
    pub max_requests: u64,
    /// Max tokens per window (across all LLM calls).
    pub max_tokens: u64,
    /// Max cost (USD) per window.
    pub max_cost_usd: f64,
    /// Window length in seconds.
    pub window_secs: u64,
    /// Fraction of the hard limit at which a soft-limit warning is raised (0..1).
    pub soft_limit_fraction: f64,
}

impl Default for QuotaSchema {
    fn default() -> Self {
        Self {
            max_requests: 1_000,
            max_tokens: 1_000_000,
            max_cost_usd: 100.0,
            window_secs: 3_600, // 1 hour
            soft_limit_fraction: 0.8,
        }
    }
}

impl QuotaSchema {
    pub fn soft_requests(&self) -> u64 {
        (self.max_requests as f64 * self.soft_limit_fraction) as u64
    }

    pub fn soft_tokens(&self) -> u64 {
        (self.max_tokens as f64 * self.soft_limit_fraction) as u64
    }

    pub fn soft_cost_usd(&self) -> f64 {
        self.max_cost_usd * self.soft_limit_fraction
    }
}
