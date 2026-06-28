use crate::sampler::LatencySampler;

#[test]
fn percentiles_correct() {
    let mut s = LatencySampler::new("test");
    for v in [10, 20, 30, 40, 50, 60, 70, 80, 90, 100] {
        s.record(v);
    }
    assert_eq!(s.p50(), Some(60)); // 10 samples: idx = round(0.5*9) = 5, sorted[5] = 60
    assert!(s.p95().unwrap() >= 90);
    assert_eq!(s.p99(), Some(100));
}

#[test]
fn empty_returns_none() {
    let s = LatencySampler::new("x");
    assert_eq!(s.mean(), None);
    assert_eq!(s.p99(), None);
}

#[test]
fn min_max_correct() {
    let mut s = LatencySampler::new("x");
    s.record(5);
    s.record(15);
    s.record(10);
    assert_eq!(s.min(), Some(5));
    assert_eq!(s.max(), Some(15));
}

#[test]
fn mean_correct() {
    let mut s = LatencySampler::new("x");
    s.record(10);
    s.record(20);
    s.record(30);
    let m = s.mean().unwrap();
    assert!((m - 20.0).abs() < 0.001);
}
