#[cfg(test)]
mod tests {
    use crate::histogram::Histogram;

    fn run_latency_hist() -> Histogram {
        Histogram::new("ancora_run_latency", vec![10, 50, 100, 500, 1000])
    }

    fn step_latency_hist() -> Histogram {
        Histogram::new("ancora_step_latency", vec![5, 25, 100, 500])
    }

    #[test]
    fn run_latency_histogram_records_buckets() {
        let mut h = run_latency_hist();
        h.observe(20);
        h.observe(80);
        h.observe(5);
        // bucket(10) catches <=10: only obs 5
        assert_eq!(h.bucket_count(10), 1);
        // bucket(50) catches <=50: obs 5 and 20
        assert_eq!(h.bucket_count(50), 2);
        // bucket(100) catches <=100: all three
        assert_eq!(h.bucket_count(100), 3);
    }

    #[test]
    fn step_latency_histogram_records_buckets() {
        let mut h = step_latency_hist();
        h.observe(3);
        h.observe(30);
        assert_eq!(h.bucket_count(5), 1);
        assert_eq!(h.bucket_count(25), 1);
        assert_eq!(h.bucket_count(100), 2);
    }

    #[test]
    fn histogram_count_and_sum() {
        let mut h = run_latency_hist();
        h.observe(10);
        h.observe(20);
        assert_eq!(h.count(), 2);
        assert_eq!(h.sum_ms(), 30);
    }

    #[test]
    fn histogram_mean() {
        let mut h = run_latency_hist();
        h.observe(10);
        h.observe(30);
        assert!((h.mean_ms() - 20.0).abs() < 1e-9);
    }

    #[test]
    fn empty_histogram_mean_is_zero() {
        let h = run_latency_hist();
        assert_eq!(h.mean_ms(), 0.0);
    }
}
