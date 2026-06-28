#[cfg(test)]
mod tests {
    use crate::ProviderRateCoordinator;

    #[test]
    fn provider_limit_blocks_excess_calls() {
        let mut coord = ProviderRateCoordinator::new();
        for _ in 0..5 {
            let _ = coord.check("acme", "openai", 5, 0);
        }
        // 6th call should fail
        let err = coord.check("acme", "openai", 5, 0).unwrap_err();
        assert!(err.is_blocking());
    }

    #[test]
    fn different_tenants_have_separate_limits() {
        let mut coord = ProviderRateCoordinator::new();
        for _ in 0..5 {
            coord.check("acme", "openai", 5, 0).ok();
        }
        // tenant-b should still have quota
        assert!(coord.check("tenant-b", "openai", 5, 0).is_ok());
    }

    #[test]
    fn provider_limit_resets_after_window() {
        let mut coord = ProviderRateCoordinator::new();
        for _ in 0..5 {
            coord.check("acme", "openai", 5, 0).ok();
        }
        // After 60s window reset
        assert!(coord.check("acme", "openai", 5, 60).is_ok());
    }
}
