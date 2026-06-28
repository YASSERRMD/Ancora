#[cfg(test)]
mod tests {
    use crate::slo::{BurnRateAlert, ErrorBudget, SloTarget};

    fn target_99_9() -> SloTarget {
        SloTarget {
            name: "api_availability".into(),
            availability_target: 0.999,
            latency_p99_ms: 500,
            window_secs: 3600,
        }
    }

    #[test]
    fn error_budget_full_when_all_success() {
        let mut b = ErrorBudget::new(target_99_9());
        for _ in 0..1000 { b.record(true); }
        assert!((b.budget_remaining_fraction() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn error_budget_computed_correctly() {
        let mut b = ErrorBudget::new(target_99_9());
        // 1000 requests, 0.5 * allowed_errors consumed
        // allowed error rate = 0.001, 0.0005 observed = 50% consumed
        for _ in 0..9995 { b.record(true); }
        for _ in 0..5 { b.record(false); }
        // 5/10000 = 0.0005 error rate, allowed 0.001 -> 50% consumed -> 50% remaining
        let remaining = b.budget_remaining_fraction();
        assert!(remaining > 0.4 && remaining < 0.6, "remaining={remaining}");
    }

    #[test]
    fn slo_breached_when_below_target() {
        let mut b = ErrorBudget::new(target_99_9());
        for _ in 0..990 { b.record(true); }
        for _ in 0..10 { b.record(false); }
        assert!(b.is_breached());
    }

    #[test]
    fn burn_rate_alert_fires() {
        let mut alert = BurnRateAlert::new(2.0);
        // observed error rate 5x the SLO error rate -> burn rate = 5.0 > 2.0 * 1.0
        alert.evaluate(0.005, 0.001);
        assert!(alert.fired);
        assert!((alert.current_burn_rate - 5.0).abs() < 1e-9);
    }

    #[test]
    fn burn_rate_alert_does_not_fire_within_threshold() {
        let mut alert = BurnRateAlert::new(2.0);
        // burn rate = 1.5 < 2.0
        alert.evaluate(0.0015, 0.001);
        assert!(!alert.fired);
    }

    #[test]
    fn burn_rate_zero_slo_error_rate_does_not_panic() {
        let mut alert = BurnRateAlert::new(2.0);
        alert.evaluate(0.01, 0.0);
        assert!(!alert.fired);
    }
}
