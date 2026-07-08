use crate::perf::{
    histogram_bucket, p95_ms, BootTimer, PhaseStopwatch, TimingSample, DEFAULT_BOOT_TARGET_MS,
};
use std::time::Duration;

#[test]
fn test_boot_to_ready_measured() {
    let mut timer = BootTimer::new(DEFAULT_BOOT_TARGET_MS);
    let sw = PhaseStopwatch::start("model-preload");
    let sample = sw.stop();
    timer.record(sample);
    let report = timer.finish();
    // Just verify it finishes and produces a summary
    let summary = report.summary();
    assert!(summary.contains("boot-to-ready"));
}

#[test]
fn test_timing_report_within_target() {
    let samples = vec![TimingSample::new("boot", Duration::from_millis(100))];
    let report = crate::perf::BootTimingReport::new(samples, Duration::from_millis(100), 5000);
    assert!(report.within_target());
}

#[test]
fn test_timing_report_exceeds_target() {
    let samples = vec![TimingSample::new("boot", Duration::from_millis(10_000))];
    let report = crate::perf::BootTimingReport::new(samples, Duration::from_millis(10_000), 5000);
    assert!(!report.within_target());
}

#[test]
fn test_slowest_phase_identified() {
    let samples = vec![
        TimingSample::new("init", Duration::from_millis(10)),
        TimingSample::new("model-preload", Duration::from_millis(2000)),
        TimingSample::new("socket-bind", Duration::from_millis(5)),
    ];
    let report = crate::perf::BootTimingReport::new(samples, Duration::from_millis(2015), 5000);
    let slowest = report.slowest_phase().unwrap();
    assert_eq!(slowest.label, "model-preload");
}

#[test]
fn test_p95_calculation() {
    let samples: Vec<Duration> = (0..100).map(Duration::from_millis).collect();
    let p95 = p95_ms(&samples);
    assert!((94..=99).contains(&p95));
}

#[test]
fn test_histogram_bucket() {
    assert_eq!(histogram_bucket(50), "<100ms");
    assert_eq!(histogram_bucket(300), "100-500ms");
    assert_eq!(histogram_bucket(750), "500ms-1s");
    assert_eq!(histogram_bucket(3000), "1s-5s");
    assert_eq!(histogram_bucket(10_000), ">5s");
}

#[test]
fn test_timing_sample_ms() {
    let s = TimingSample::new("x", Duration::from_millis(250));
    assert_eq!(s.ms(), 250);
}
