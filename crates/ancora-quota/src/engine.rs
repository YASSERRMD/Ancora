use std::collections::HashMap;
use crate::error::QuotaError;
use crate::rate_limiter::RateLimiter;
use crate::schema::QuotaSchema;
use crate::usage::QuotaUsage;

/// Central quota engine. One `RateLimiter` per tenant.
#[derive(Default)]
pub struct QuotaEngine {
    limiters: HashMap<String, RateLimiter>,
}

impl QuotaEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_tenant(&mut self, tenant: &str, schema: QuotaSchema, now: u64) {
        self.limiters.insert(tenant.to_owned(), RateLimiter::new(tenant, schema, now));
    }

    /// Check and record a usage event. Soft-limit warning is returned as `Err` but
    /// is non-blocking (caller decides whether to log and continue).
    pub fn check(&mut self, tenant: &str, tokens: u64, cost_usd: f64, now: u64) -> Result<(), QuotaError> {
        if let Some(limiter) = self.limiters.get_mut(tenant) {
            limiter.check_and_record(tokens, cost_usd, now)
        } else {
            Ok(())
        }
    }

    pub fn usage(&mut self, tenant: &str, now: u64) -> Option<QuotaUsage> {
        let limiter = self.limiters.get_mut(tenant)?;
        Some(QuotaUsage {
            tenant: tenant.to_owned(),
            requests: limiter.request_count(now),
            max_requests: limiter.schema.max_requests,
            tokens: limiter.token_count(now),
            max_tokens: limiter.schema.max_tokens,
            cost_usd: limiter.cost_total(now),
            max_cost_usd: limiter.schema.max_cost_usd,
        })
    }
}
