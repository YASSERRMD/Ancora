#[cfg(test)]
mod tests {
    use crate::QuotaSchema;

    #[test]
    fn soft_limits_derived_from_fraction() {
        let schema = QuotaSchema {
            max_requests: 1000,
            max_tokens: 2000,
            max_cost_usd: 10.0,
            window_secs: 3600,
            soft_limit_fraction: 0.8,
        };
        assert_eq!(schema.soft_requests(), 800);
        assert_eq!(schema.soft_tokens(), 1600);
        assert!((schema.soft_cost_usd() - 8.0).abs() < 1e-9);
    }

    #[test]
    fn default_schema_has_sane_values() {
        let schema = QuotaSchema::default();
        assert!(schema.max_requests > 0);
        assert!(schema.max_tokens > 0);
        assert!(schema.max_cost_usd > 0.0);
        assert!(schema.soft_limit_fraction > 0.0 && schema.soft_limit_fraction < 1.0);
    }
}
