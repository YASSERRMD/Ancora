#[cfg(test)]
mod tests {
    use crate::{QuotaEngine, QuotaSchema};

    fn tight_schema() -> QuotaSchema {
        QuotaSchema {
            max_requests: 2,
            max_tokens: 50,
            max_cost_usd: 0.5,
            window_secs: 60,
            soft_limit_fraction: 0.9,
        }
    }

    #[test]
    fn engine_blocks_over_limit_tenant() {
        let mut engine = QuotaEngine::new();
        engine.register_tenant("acme", tight_schema(), 0);
        engine.check("acme", 10, 0.1, 0).ok();
        engine.check("acme", 10, 0.1, 0).ok();
        let err = engine.check("acme", 10, 0.1, 0).unwrap_err();
        assert!(err.is_blocking());
    }

    #[test]
    fn engine_allows_unregistered_tenant() {
        let mut engine = QuotaEngine::new();
        assert!(engine.check("ghost", 100, 10.0, 0).is_ok());
    }

    #[test]
    fn distributed_counters_agree_across_calls() {
        let mut engine = QuotaEngine::new();
        engine.register_tenant("acme", tight_schema(), 0);
        engine.check("acme", 10, 0.0, 0).ok();
        engine.check("acme", 10, 0.0, 0).ok();
        let usage = engine.usage("acme", 0).unwrap();
        assert_eq!(usage.requests, 2);
        assert_eq!(usage.tokens, 20);
    }
}
