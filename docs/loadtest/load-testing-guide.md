# Ancora Load Testing Guide

The `ancora-loadtest` crate provides a pure-Rust, offline load testing harness for validating production readiness.

## Defining a workload

```rust
use ancora_loadtest::WorkloadSpec;

let spec = WorkloadSpec::new("inference-baseline", 100.0, 300, 10)
    .with_payload(512); // 512-byte payloads
println!("Expected requests: {}", spec.total_expected_requests()); // 30_000
```

## Running a soak test

```rust
use ancora_loadtest::SoakHarness;

let mut harness = SoakHarness::new("soak-30min", 1800, now_secs);
while !harness.is_complete(now_secs) {
    let (latency_ms, had_error) = dispatch_request();
    harness.record(latency_ms, had_error, now_secs);
    now_secs += 1;
}
harness.finish(now_secs);

let summary = harness.summary();
println!("p99={} error_rate={:.2}%", summary.p99_ms, summary.error_rate * 100.0);

if summary.passes_slo(0.001, 2000) {
    println!("PASS");
}
```

## Composing scenarios into a report

```rust
use ancora_loadtest::{Scenario, WorkloadSpec, LoadTestReport, ScenarioReport};

let scenarios = vec![
    Scenario::new(WorkloadSpec::new("baseline", 100.0, 60, 10), 0.01, 500),
    Scenario::new(WorkloadSpec::new("spike", 500.0, 10, 50), 0.05, 1000),
];

let mut reports = vec![];
for scenario in scenarios {
    let result = scenario.run(|_| simulate_request());
    reports.push(ScenarioReport::from_summary(&result.name, result.passed, &result.summary));
}

let report = LoadTestReport::new(reports);
println!("{}", report.to_json());
assert!(report.all_passed);
```

## Interpreting percentiles

| Metric | Meaning |
|--------|---------|
| p50 | Median latency - typical user experience |
| p95 | 95th percentile - most users' worst case |
| p99 | 99th percentile - SLO target for tail latency |

A failing p99 with a passing p95 indicates a small percentage of requests hitting a slow path (lock contention, cold cache, retry).
