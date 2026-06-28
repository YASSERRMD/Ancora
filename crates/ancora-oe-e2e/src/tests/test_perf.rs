/// Performance tests: measure observability overhead.

use crate::perf::{measure_span_overhead, BenchmarkReport};

#[test]
fn observability_overhead_is_measured() {
    let measurement = measure_span_overhead(1000);

    // We just verify the measurement ran and produced valid numbers.
    // We do not assert a specific overhead bound since CI hardware varies.
    assert!(measurement.baseline_ns > 0 || measurement.instrumented_ns > 0,
        "at least one timing must be non-zero");
    // Overhead fraction must be finite.
    let fraction = measurement.overhead_fraction();
    assert!(fraction.is_finite(), "overhead fraction must be finite");
}

#[test]
fn benchmark_report_computes_mean_overhead() {
    use crate::perf::OverheadMeasurement;

    let mut report = BenchmarkReport::new();
    report.add(OverheadMeasurement::new("op1", 1000, 1100));
    report.add(OverheadMeasurement::new("op2", 2000, 2200));

    let mean_pct = report.mean_overhead_pct();
    assert!(mean_pct >= 0.0, "mean overhead must be non-negative");
    // op1: 10% overhead, op2: 10% overhead -> mean = 10%.
    assert!((mean_pct - 10.0).abs() < 1e-9, "mean overhead must be 10%");
}

#[test]
fn benchmark_report_worst_case_is_correct() {
    use crate::perf::OverheadMeasurement;

    let mut report = BenchmarkReport::new();
    report.add(OverheadMeasurement::new("fast", 1000, 1010));   // 1% overhead
    report.add(OverheadMeasurement::new("slow", 1000, 1200));   // 20% overhead

    let worst = report.worst_case().unwrap();
    assert_eq!(worst.label, "slow");
}

#[test]
fn overhead_within_budget_check_works() {
    use crate::perf::OverheadMeasurement;

    let low = OverheadMeasurement::new("low", 1000, 1020);   // 2% overhead
    let high = OverheadMeasurement::new("high", 1000, 1200); // 20% overhead

    assert!(low.within_budget(0.05), "2% is within 5% budget");
    assert!(!high.within_budget(0.05), "20% exceeds 5% budget");
}
