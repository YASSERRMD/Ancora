#[cfg(test)]
mod tests {
    use crate::{QuotaError, QuotaSchema, RateLimiter};

    fn tight_schema() -> QuotaSchema {
        QuotaSchema {
            max_requests: 3,
            max_tokens: 100,
            max_cost_usd: 1.0,
            window_secs: 60,
            soft_limit_fraction: 0.8, // soft at 3*0.8=2.4 => 2
        }
    }

    #[test]
    fn rate_limit_blocks_over_limit_requests() {
        let mut rl = RateLimiter::new("acme", tight_schema(), 0);
        // First 3 succeed (though 3rd hits soft)
        let _ = rl.check_and_record(10, 0.1, 0);
        let _ = rl.check_and_record(10, 0.1, 0);
        let _ = rl.check_and_record(10, 0.1, 0);
        // 4th should hard-block
        let err = rl.check_and_record(10, 0.1, 0).unwrap_err();
        assert!(err.is_blocking(), "4th request must be blocked");
    }

    #[test]
    fn token_budget_enforced() {
        let mut rl = RateLimiter::new("acme", tight_schema(), 0);
        // Use 90 tokens - still under 100
        let _ = rl.check_and_record(90, 0.0, 0);
        // Next call wants 20 more (90+20=110 > 100)
        let err = rl.check_and_record(20, 0.0, 0).unwrap_err();
        assert!(err.is_blocking());
    }

    #[test]
    fn cost_budget_enforced() {
        let mut rl = RateLimiter::new("acme", tight_schema(), 0);
        let _ = rl.check_and_record(1, 0.9, 0);
        // Next 0.2 would push cost to 1.1 > 1.0
        let err = rl.check_and_record(1, 0.2, 0).unwrap_err();
        assert!(err.is_blocking());
    }

    #[test]
    fn soft_limit_warns_hard_limit_blocks() {
        let schema = QuotaSchema {
            max_requests: 5,
            soft_limit_fraction: 0.6, // soft at 3
            ..QuotaSchema::default()
        };
        let mut rl = RateLimiter::new("acme", schema, 0);
        let _ = rl.check_and_record(1, 0.0, 0);
        let _ = rl.check_and_record(1, 0.0, 0);
        // 3rd hits soft
        let soft = rl.check_and_record(1, 0.0, 0).unwrap_err();
        assert!(!soft.is_blocking(), "soft limit must be non-blocking");
    }

    #[test]
    fn budget_resets_at_window() {
        let mut rl = RateLimiter::new("acme", tight_schema(), 0);
        let _ = rl.check_and_record(30, 0.0, 0);
        let _ = rl.check_and_record(30, 0.0, 0);
        // Advance past the window
        let result = rl.check_and_record(30, 0.0, 60);
        // After reset, 30 tokens should be fine
        assert!(result.is_ok() || matches!(result, Err(QuotaError::SoftLimitWarning { .. })));
    }

    #[test]
    fn retry_after_provided_on_hard_limit() {
        let mut rl = RateLimiter::new("acme", tight_schema(), 0);
        for _ in 0..3 {
            let _ = rl.check_and_record(1, 0.0, 0);
        }
        let err = rl.check_and_record(1, 0.0, 0).unwrap_err();
        assert!(err.retry_after_secs().unwrap_or(0) > 0);
    }
}
