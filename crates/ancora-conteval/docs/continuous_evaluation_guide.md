# Continuous Evaluation Guide

This document describes how to use `ancora-conteval` to track model and provider
quality over time in the Ancora agent framework.

## Overview

Continuous evaluation runs evaluation jobs on a recurring schedule, sampling
production traffic, computing rolling quality metrics, detecting quality trends,
and raising alerts when quality drops below configured thresholds.

## Components

### Scheduler

The `EvalScheduler` registers `EvalJob` instances and determines which jobs are
due at a given moment. Each job specifies an `EvalInterval` (seconds, minutes,
or hours). Jobs that have never run are immediately due.

```rust
use ancora_conteval::scheduler::{EvalInterval, EvalJob, EvalScheduler};
use std::time::SystemTime;

let mut sched = EvalScheduler::new();
sched.register(EvalJob::new("hourly-eval", EvalInterval::Hours(1)));
let due = sched.due_jobs(SystemTime::now());
```

### Production Samples

`ProdEvalSet` ingests production traffic samples. Samples containing PII must be
redacted before ingestion - the set will reject any un-redacted PII samples.

```rust
use ancora_conteval::prod_samples::{ProdEvalSet, ProdSample};

let mut set = ProdEvalSet::new();
let mut sample = ProdSample::new("id", "gpt-4", "openai", "text", "reply", 120)
    .with_pii();
sample.redact("[REDACTED]");
set.add(sample).unwrap();
```

### Rolling Metrics

`RollingMetric` maintains a fixed-size ring buffer of quality scores. Older
entries are evicted as new ones arrive. It provides mean, min, max, and standard
deviation over the active window.

### Model and Provider Tracking

`ModelTracker` and `ProviderTracker` aggregate rolling metrics per model and per
provider respectively, enabling comparison across models and between providers.

### Trend Detection

`TrendDetector` fits a linear regression over the most recent N scores and
classifies the trend as `Improving`, `Stable`, or `Degrading`.

### Dataset Refresh

`DatasetRefresher` enforces a `RefreshPolicy` that evicts stale samples and
trims the dataset to a configured maximum size. Call `refresh()` when
`needs_refresh()` returns true.

### Alerting

`AlertEngine` raises `QualityAlert` instances when:
- A score falls below the warning or critical threshold.
- A score drops sharply relative to the previous observation.
- A degrading trend slope is detected.

### Dashboard JSON

`DashboardState::to_json()` produces a JSON string summarising model and
provider quality plus the alert summary. Use `validate_json()` to verify
the output before forwarding to downstream consumers.
