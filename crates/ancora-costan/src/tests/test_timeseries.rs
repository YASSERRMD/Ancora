use crate::timeseries::CostTimeSeries;

#[test]
fn timeseries_records_and_sums() {
    let mut ts = CostTimeSeries::new();
    ts.record(1000, 1.00, 100);
    ts.record(2000, 0.50, 50);
    ts.record(3000, 0.25, 25);
    let total = ts.total_cost();
    assert!((total - 1.75).abs() < 1e-9, "expected 1.75, got {}", total);
}

#[test]
fn timeseries_sorted_by_timestamp() {
    let mut ts = CostTimeSeries::new();
    ts.record(3000, 0.3, 30);
    ts.record(1000, 0.1, 10);
    ts.record(2000, 0.2, 20);
    let points = ts.points();
    assert_eq!(points[0].timestamp, 1000);
    assert_eq!(points[1].timestamp, 2000);
    assert_eq!(points[2].timestamp, 3000);
}

#[test]
fn rolling_avg_last_two() {
    let mut ts = CostTimeSeries::new();
    ts.record(1000, 1.0, 100);
    ts.record(2000, 3.0, 300);
    ts.record(3000, 5.0, 500);
    let avg = ts.rolling_avg(2).unwrap();
    // last 2: 3.0, 5.0 -> avg 4.0
    assert!((avg - 4.0).abs() < 1e-9);
}

#[test]
fn hourly_buckets_aggregated() {
    let mut ts = CostTimeSeries::new();
    // both within the same hour (t=0 to 3599)
    ts.record(100, 1.0, 100);
    ts.record(200, 2.0, 200);
    // different hour
    ts.record(3700, 0.5, 50);
    let buckets = ts.hourly_buckets();
    assert_eq!(buckets.len(), 2);
    let first_bucket_cost = buckets[0].1;
    assert!((first_bucket_cost - 3.0).abs() < 1e-9);
}
