#[cfg(test)]
mod tests {
    use crate::{dedup::AlertDedup, rules::check_high_error_rate};

    #[test]
    fn dedup_routes_first_alert() {
        let mut d = AlertDedup::new(300);
        let a = check_high_error_rate(0.10, 0.05, 1000).unwrap();
        assert!(d.should_route(&a));
    }

    #[test]
    fn dedup_suppresses_duplicate_within_cooldown() {
        let mut d = AlertDedup::new(300);
        let a1 = check_high_error_rate(0.10, 0.05, 1000).unwrap();
        let a2 = check_high_error_rate(0.10, 0.05, 1100).unwrap();
        assert!(d.should_route(&a1));
        // same fingerprint within 300s cooldown
        assert!(!d.should_route(&a2));
    }

    #[test]
    fn dedup_allows_alert_after_cooldown() {
        let mut d = AlertDedup::new(300);
        let a1 = check_high_error_rate(0.10, 0.05, 1000).unwrap();
        let a2 = check_high_error_rate(0.10, 0.05, 1400).unwrap();
        assert!(d.should_route(&a1));
        assert!(d.should_route(&a2));
    }
}
