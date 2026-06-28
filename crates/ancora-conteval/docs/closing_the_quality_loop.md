# Closing the Quality Loop

This document explains how the components of `ancora-conteval` fit together
to form a closed-loop quality system that continuously improves model and
provider reliability.

## The Feedback Loop

```
Production Traffic
       |
       v
  ProdEvalSet  <--- RefreshPolicy (evict stale, cap size)
       |
       v
  Evaluation Run (scheduled by EvalScheduler)
       |
       v
  Quality Scores
       |
       +-----> ModelTracker (per-model rolling mean)
       |
       +-----> ProviderTracker (per-provider rolling mean)
       |
       v
  TrendDetector (linear regression over window)
       |
       v
  AlertEngine (threshold + sudden-drop + trend alerts)
       |
       v
  DashboardState (JSON snapshot for monitoring UIs)
```

## Key Design Decisions

### Redaction Before Evaluation

Production samples may contain PII. The pipeline enforces that any sample
marked as containing PII is redacted before it enters the evaluation set.
This is a hard reject - `ProdEvalSet::add()` returns an error for un-redacted
PII samples. This ensures user data privacy is maintained throughout the
evaluation pipeline.

### Rolling Windows

Quality metrics are computed over a sliding window rather than a cumulative
average. This means recent quality changes have more impact than historical
data, making the system responsive to sudden regressions.

### Linear Regression for Trends

A simple linear regression over the N most recent mean scores provides a
noise-tolerant trend signal. The slope of the fitted line indicates the
rate of quality change per evaluation step. Thresholds on the slope magnitude
separate stable operation from meaningful improvement or degradation.

### Atomic Alerting

Alerts are generated and held in the `AlertEngine` until drained by the
caller. Draining is a destructive read - alerts are removed from the engine
and returned to the caller for forwarding to alerting sinks (PagerDuty,
Slack, etc.) without risk of double-delivery.

### Zero External Dependencies

All components are implemented using only the Rust standard library. This
ensures the crate compiles in offline and air-gapped environments and
adds no transitive dependency risk to the Ancora workspace.

## Operational Guidance

1. Configure `RefreshPolicy` with a `max_sample_age` that matches your traffic
   volume - busier deployments can afford shorter windows.
2. Set `warning_threshold` and `critical_threshold` based on your SLO baseline.
3. Tune `degrading_slope` by observing natural score variance during stable
   operation; start conservative (e.g. 0.02) and tighten as you learn the
   system's noise floor.
4. Run the dashboard JSON endpoint on a short interval (e.g. 30 seconds) so
   monitoring UIs reflect near-real-time quality state.
