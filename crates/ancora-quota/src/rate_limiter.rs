use crate::error::QuotaError;
use crate::schema::QuotaSchema;
use crate::window::SlidingWindow;

/// Per-tenant sliding-window rate limiter covering requests, tokens, and cost.
pub struct RateLimiter {
    pub tenant: String,
    pub schema: QuotaSchema,
    requests: SlidingWindow,
    tokens: SlidingWindow,
    cost: f64,
    cost_window_start: u64,
}

impl RateLimiter {
    pub fn new(tenant: impl Into<String>, schema: QuotaSchema, now: u64) -> Self {
        let ws = schema.window_secs;
        Self {
            tenant: tenant.into(),
            schema,
            requests: SlidingWindow::new(ws, now),
            tokens: SlidingWindow::new(ws, now),
            cost: 0.0,
            cost_window_start: now,
        }
    }

    fn reset_cost_if_needed(&mut self, now: u64) {
        if now >= self.cost_window_start + self.schema.window_secs {
            self.cost = 0.0;
            self.cost_window_start = now;
        }
    }

    /// Check and record one request consuming `token_count` tokens and `cost_usd` cost.
    /// Returns `Err(HardLimitExceeded)` if any hard limit would be breached.
    /// Returns `Err(SoftLimitWarning)` if a soft limit is crossed (non-blocking).
    pub fn check_and_record(&mut self, token_count: u64, cost_usd: f64, now: u64) -> Result<(), QuotaError> {
        self.reset_cost_if_needed(now);

        let new_reqs = self.requests.value(now) + 1;
        let new_tokens = self.tokens.value(now) + token_count;
        let new_cost = self.cost + cost_usd;
        let retry = self.requests.seconds_until_reset(now).max(1);

        if new_reqs > self.schema.max_requests {
            return Err(QuotaError::HardLimitExceeded {
                tenant: self.tenant.clone(),
                retry_after_secs: retry,
            });
        }
        if new_tokens > self.schema.max_tokens {
            return Err(QuotaError::HardLimitExceeded {
                tenant: self.tenant.clone(),
                retry_after_secs: retry,
            });
        }
        if new_cost > self.schema.max_cost_usd {
            return Err(QuotaError::HardLimitExceeded {
                tenant: self.tenant.clone(),
                retry_after_secs: retry,
            });
        }

        self.requests.increment(now, 1);
        self.tokens.increment(now, token_count);
        self.cost = new_cost;

        // Soft-limit warnings (non-blocking)
        if new_reqs >= self.schema.soft_requests() {
            let pct = (new_reqs as f64 / self.schema.max_requests as f64) * 100.0;
            return Err(QuotaError::SoftLimitWarning {
                tenant: self.tenant.clone(),
                metric: "requests".to_string(),
                pct,
            });
        }
        Ok(())
    }

    pub fn request_count(&mut self, now: u64) -> u64 {
        self.requests.value(now)
    }

    pub fn token_count(&mut self, now: u64) -> u64 {
        self.tokens.value(now)
    }

    pub fn cost_total(&mut self, now: u64) -> f64 {
        self.reset_cost_if_needed(now);
        self.cost
    }
}
