use crate::soak::SoakHarness;

#[test]
fn harness_records_samples() {
    let mut h = SoakHarness::new("test", 60, 0);
    h.record(10, false, 0);
    h.record(20, false, 1);
    h.record(30, true, 2);
    let s = h.summary();
    assert_eq!(s.total_requests, 3);
    assert_eq!(s.total_errors, 1);
}

#[test]
fn is_complete_after_duration() {
    let h = SoakHarness::new("t", 30, 0);
    assert!(!h.is_complete(20));
    assert!(h.is_complete(30));
}

#[test]
fn passes_slo_within_thresholds() {
    let mut h = SoakHarness::new("t", 10, 0);
    for _ in 0..100 {
        h.record(50, false, 0);
    }
    let s = h.summary();
    assert!(s.passes_slo(0.01, 200));
}

#[test]
fn fails_slo_on_high_p99() {
    let mut h = SoakHarness::new("t", 10, 0);
    // 90 fast + 10 slow: p99 idx = round(0.99 * 99) = 98, which hits the slow region
    for _ in 0..90 {
        h.record(10, false, 0);
    }
    for _ in 0..10 {
        h.record(5000, false, 0);
    }
    let s = h.summary();
    assert!(!s.passes_slo(0.01, 200));
}
