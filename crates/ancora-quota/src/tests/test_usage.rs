#[cfg(test)]
mod tests {
    use crate::{QuotaEngine, QuotaSchema};

    #[test]
    fn usage_query_accurate() {
        let mut engine = QuotaEngine::new();
        let schema = QuotaSchema {
            max_requests: 100,
            max_tokens: 1_000,
            max_cost_usd: 10.0,
            window_secs: 3600,
            soft_limit_fraction: 0.8,
        };
        engine.register_tenant("acme", schema, 0);
        engine.check("acme", 200, 2.0, 0).ok();
        engine.check("acme", 300, 1.5, 0).ok();
        let usage = engine.usage("acme", 0).unwrap();
        assert_eq!(usage.requests, 2);
        assert_eq!(usage.tokens, 500);
        assert!((usage.cost_usd - 3.5).abs() < 1e-9);
    }

    #[test]
    fn usage_percentages_calculated_correctly() {
        let mut engine = QuotaEngine::new();
        let schema = QuotaSchema {
            max_requests: 100,
            max_tokens: 1_000,
            max_cost_usd: 10.0,
            window_secs: 3600,
            soft_limit_fraction: 0.8,
        };
        engine.register_tenant("acme", schema, 0);
        for _ in 0..50 {
            engine.check("acme", 10, 0.05, 0).ok();
        }
        let usage = engine.usage("acme", 0).unwrap();
        assert!((usage.request_pct() - 50.0).abs() < 1.0);
        assert!((usage.token_pct() - 50.0).abs() < 1.0);
        assert!((usage.cost_pct() - 25.0).abs() < 1.0);
    }
}
