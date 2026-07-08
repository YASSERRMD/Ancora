use crate::cache_savings::CacheSavingsTracker;

#[test]
fn cache_savings_tracked_correctly() {
    let mut t = CacheSavingsTracker::new();

    // 3 cache hits: would have cost 2.0 each, paid 0.2 each
    for _ in 0..3 {
        t.record_hit(2.0, 0.2, 500);
    }
    // 1 cache miss: full cost
    t.record_miss(2.0, 500);

    let savings = t.total_savings();
    // savings = 3 * (2.0 - 0.2) = 5.4
    assert!(
        (savings - 5.4).abs() < 1e-9,
        "savings should be 5.4, got {}",
        savings
    );
}

#[test]
fn hit_rate_calculated() {
    let mut t = CacheSavingsTracker::new();
    t.record_hit(1.0, 0.1, 100);
    t.record_hit(1.0, 0.1, 100);
    t.record_miss(1.0, 100);
    let rate = t.hit_rate();
    // 2 hits out of 3 = 0.666...
    assert!((rate - 2.0 / 3.0).abs() < 1e-9);
}

#[test]
fn saving_percentage_correct() {
    let mut t = CacheSavingsTracker::new();
    t.record_hit(10.0, 2.0, 1000); // saves 8.0
    t.record_miss(2.0, 200); // no savings, full_cost = 2.0
                             // total savings = 8.0, full_cost = 12.0 => 66.67%
    let pct = t.saving_percentage();
    assert!((pct - (8.0 / 12.0 * 100.0)).abs() < 1e-6);
}

#[test]
fn no_hits_no_savings() {
    let mut t = CacheSavingsTracker::new();
    t.record_miss(5.0, 500);
    assert_eq!(t.total_savings(), 0.0);
    assert_eq!(t.hit_rate(), 0.0);
}
