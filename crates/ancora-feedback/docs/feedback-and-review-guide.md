# Feedback and Review Guide

## Overview

The `ancora-feedback` crate captures human feedback on agent runs and routes
low-confidence runs through a structured review workflow.

## Submitting Feedback

Use the `FeedbackApi` to submit thumbs-up or thumbs-down ratings with an
optional free-text comment:

```rust
use ancora_feedback::api::{FeedbackApi, SubmitFeedbackRequest};
use ancora_feedback::schema::ThumbsRating;

let mut api = FeedbackApi::new(0.7); // confidence threshold
let resp = api.submit_feedback(
    SubmitFeedbackRequest {
        run_id: "run-abc".into(),
        step_id: None,
        rating: ThumbsRating::Down,
        comment: Some("Response contained incorrect facts".into()),
        author: "reviewer@example.com".into(),
    },
    0.45, // run's confidence score
    timestamp_now(),
);

if resp.queued_for_review {
    println!("Run {} queued for human review", resp.feedback_id);
}
```

## Low-Confidence Review Queue

Any run with a confidence score below the configured threshold is automatically
added to the `ReviewQueue`. Reviewers can:

1. Poll `api.pending_reviews()` for unclaimed entries.
2. Claim an entry via `queue.claim(run_id)`.
3. Submit a `ReviewDecision` (Approve, Reject, or RequestChanges).

## Schema

- `ThumbsRating` - Up or Down
- `Feedback` - id, run_id, step_id, rating, comment, author, created_at
- `ReviewDecision` - run_id, reviewer_id, outcome, notes, decided_at
