/// Feedback schema: rating, comment, feedback record.
pub mod schema;

/// Attach feedback to runs and steps.
pub mod attach;

/// Low-confidence review queue.
pub mod queue;

/// Reviewer assignment.
pub mod reviewer;

/// Review decision capture.
pub mod decision;

/// Feedback to eval dataset pipeline.
pub mod pipeline;

/// Feedback aggregation metrics.
pub mod aggregation;

/// Feedback-driven guardrail tuning input.
pub mod tuning;

/// Feedback API surface.
pub mod api;

#[cfg(test)]
mod tests;
