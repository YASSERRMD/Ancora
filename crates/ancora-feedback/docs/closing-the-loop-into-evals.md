# Closing the Loop into Evals

## Overview

Human feedback collected during production runs is converted into eval dataset
entries via the `FeedbackToEvalPipeline`. This closes the loop between user
signals and continuous model/guardrail improvement.

## Pipeline

```rust
use ancora_feedback::pipeline::FeedbackToEvalPipeline;

let mut pipeline = FeedbackToEvalPipeline::new();

// Ingest each feedback record
for fb in feedback_store.for_run("run-abc") {
    pipeline.ingest(fb);
}

// Drain eval cases for export
let cases = pipeline.drain();
for case in &cases {
    println!("Eval case {} - positive: {}", case.id, case.is_positive);
}
```

## Aggregation and Tuning

The `aggregation` module computes approval rates and comment density. These
metrics feed the `tuning` module, which generates `TuningSignal` records:

```rust
use ancora_feedback::aggregation::aggregate;
use ancora_feedback::tuning::{TuningConfig, derive_tuning_signal};

let metrics = aggregate(&all_feedback);
let config = TuningConfig::default();
if let Some(signal) = derive_tuning_signal("toxicity-guard", &metrics, &config) {
    println!("Tuning signal: {} (adjustment: {})", signal.target, signal.adjustment);
}
```

## Feedback Loop Summary

| Stage               | Module        | Output                  |
|---------------------|---------------|-------------------------|
| Collect feedback    | `schema`      | `Feedback` records      |
| Attach to runs      | `attach`      | `FeedbackStore`         |
| Queue low-conf runs | `queue`       | `ReviewQueue` entries   |
| Assign reviewers    | `reviewer`    | `Assignment` records    |
| Capture decisions   | `decision`    | `ReviewDecision`        |
| Convert to evals    | `pipeline`    | `EvalCase` records      |
| Aggregate metrics   | `aggregation` | `FeedbackMetrics`       |
| Drive tuning        | `tuning`      | `TuningSignal`          |
