# Review Workflow

## Overview

When an agent run scores below the confidence threshold, it enters the review
workflow. This document describes the lifecycle of a queued run from submission
through reviewer decision.

## Lifecycle

```
Run completes (low confidence)
        |
        v
ReviewQueue.submit(run_id, confidence)
        |
        v
Run paused / flagged
        |
        v
Reviewer polls pending_reviews()
        |
        v
ReviewerRegistry.assign(run_id, reviewer_id)
        |
        v
Reviewer examines run output
        |
        +-- Approve       --> Run resumes, output released
        |
        +-- Reject        --> Run halted, flagged for re-generation
        |
        +-- RequestChanges --> Run remains paused, revision requested
```

## Reviewer Assignment

```rust
use ancora_feedback::reviewer::{Reviewer, ReviewerRegistry};

let mut registry = ReviewerRegistry::new();
registry.register(Reviewer {
    id: "rev-1".into(),
    name: "Alice Smith".into(),
    email: "alice@example.com".into(),
});
registry.assign("run-xyz", "rev-1").unwrap();
```

## Capturing Decisions

```rust
use ancora_feedback::api::SubmitDecisionRequest;
use ancora_feedback::decision::DecisionOutcome;

api.submit_decision(
    SubmitDecisionRequest {
        run_id: "run-xyz".into(),
        reviewer_id: "rev-1".into(),
        outcome: DecisionOutcome::Approve,
        notes: Some("Verified against source documents".into()),
    },
    timestamp_now(),
);
```
