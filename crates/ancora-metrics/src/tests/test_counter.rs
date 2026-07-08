#[cfg(test)]
mod tests {
    use crate::counter::RunCounters;

    #[test]
    fn counters_increment_correctly() {
        let mut c = RunCounters::default();
        c.record_success("t1");
        c.record_success("t1");
        c.record_failure("t1");
        assert_eq!(c.success_total("t1"), 2);
        assert_eq!(c.failure_total("t1"), 1);
        assert_eq!(c.total("t1"), 3);
    }

    #[test]
    fn error_rate_computed() {
        let mut c = RunCounters::default();
        for _ in 0..9 {
            c.record_success("t1");
        }
        c.record_failure("t1");
        let rate = c.error_rate("t1");
        assert!((rate - 0.1).abs() < 1e-9);
    }

    #[test]
    fn zero_total_gives_zero_rate() {
        let c = RunCounters::default();
        assert_eq!(c.error_rate("none"), 0.0);
    }

    #[test]
    fn counters_are_tenant_scoped() {
        let mut c = RunCounters::default();
        c.record_success("a");
        c.record_failure("b");
        assert_eq!(c.success_total("a"), 1);
        assert_eq!(c.failure_total("a"), 0);
        assert_eq!(c.failure_total("b"), 1);
    }
}
