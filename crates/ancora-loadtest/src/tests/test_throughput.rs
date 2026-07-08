use crate::throughput::ThroughputTracker;

#[test]
fn rps_in_bucket_correct() {
    let mut t = ThroughputTracker::new(10);
    for i in 0..5 {
        t.record_ok(i);
    }
    let rps = t.rps_in_bucket(0);
    assert!((rps - 0.5).abs() < 0.001);
}

#[test]
fn error_rate_correct() {
    let mut t = ThroughputTracker::new(10);
    t.record_ok(0);
    t.record_ok(0);
    t.record_error(0);
    assert!((t.error_rate() - 1.0 / 3.0).abs() < 0.001);
}

#[test]
fn peak_rps_returns_highest_bucket() {
    let mut t = ThroughputTracker::new(10);
    for i in 0..10 {
        t.record_ok(i);
    } // bucket 0: 10 req
    for i in 20..25 {
        t.record_ok(i);
    } // bucket 2: 5 req
    assert!((t.peak_rps() - 1.0).abs() < 0.001);
}

#[test]
fn zero_requests_zero_error_rate() {
    let t = ThroughputTracker::new(1);
    assert_eq!(t.error_rate(), 0.0);
}
