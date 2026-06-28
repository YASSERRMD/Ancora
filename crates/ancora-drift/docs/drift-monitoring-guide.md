# Drift Monitoring Guide

ancora-drift monitors production quality by comparing live request/response
distributions against a captured reference baseline.

## Concepts

- **Reference distribution** - statistical summary of a healthy baseline window.
- **Drift detector** - compares a current window against the reference and
  produces a result with a z-score or absolute difference.
- **Alert** - structured signal emitted when drift exceeds a threshold.
- **Sampler** - reservoir sampler that captures a fraction of live traces for
  offline evaluation.

## Quick start

```rust
use ancora_drift::{
    reference::ReferenceBuilder,
    input_drift::InputDriftDetector,
    alerting::{Alert, AlertAggregator, Severity},
};

// 1. Build a reference from a healthy baseline.
let mut builder = ReferenceBuilder::new();
for (input, output) in baseline_traces {
    builder.add(input, output, cost, latency_ms, &tools, provider);
}
let reference = builder.build().expect("non-empty baseline");

// 2. Collect current-window stats.
let current_stats = compute_current_input_stats();

// 3. Check for drift.
let detector = InputDriftDetector::new(3.0);
let result = detector.check(&reference, &current_stats).unwrap();

// 4. Alert if needed.
let mut agg = AlertAggregator::new();
agg.push_if(
    result.drifted,
    Alert::new(Severity::Warning, "input_drift", "input distribution shifted")
        .with_metric(result.mean_z_score),
);
```

## Available detectors

| Detector | Module | Metric compared |
|---|---|---|
| Input drift | `input_drift` | Input character length distribution |
| Output drift | `output_drift` | Output character length distribution |
| Tool drift | `tool_drift` | Per-tool call frequency |
| Cost drift | `cost_drift` | Per-request cost in micro-dollars |
| Provider change | `provider_change` | Provider share and latency |
