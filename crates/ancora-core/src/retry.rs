use crate::error::AncoraError;

/// Whether an error should be retried or immediately surfaced to the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    /// The operation can be tried again after a delay.
    Retryable,
    /// The error is permanent; retrying will not help.
    Terminal,
}

/// Classify an `AncoraError` as retryable or terminal.
///
/// Network/transient errors (HTTP failures, model unreachable, timeouts)
/// are retryable. Logic/contract errors (policy violations, invalid input,
/// nondeterminism, internal errors) are terminal.
pub fn classify(error: &AncoraError) -> ErrorClass {
    match error {
        AncoraError::ModelHttp { .. }
        | AncoraError::ModelUnreachable(_)
        | AncoraError::Timeout { .. }
        | AncoraError::Storage(_)
        | AncoraError::ToolFailed { .. } => ErrorClass::Retryable,

        AncoraError::Nondeterminism { .. }
        | AncoraError::JournalGap { .. }
        | AncoraError::JournalWrite(_)
        | AncoraError::MaxSteps { .. }
        | AncoraError::OutputValidation { .. }
        | AncoraError::ModelRefused(_)
        | AncoraError::ModelParse(_)
        | AncoraError::ToolNotFound(_)
        | AncoraError::ToolInputInvalid { .. }
        | AncoraError::ToolDenied(_)
        | AncoraError::PolicyResidency(_)
        | AncoraError::PolicyPermission(_)
        | AncoraError::GraphInvalid(_)
        | AncoraError::NodeNotFound(_)
        | AncoraError::Cancelled(_)
        | AncoraError::InvalidState(_)
        | AncoraError::Internal(_) => ErrorClass::Terminal,
    }
}

/// Policy that controls how many times an operation is retried and how
/// long to wait between attempts.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of attempts (including the first). Must be >= 1.
    pub max_attempts: u32,
    /// Base delay in milliseconds before the first retry.
    pub initial_backoff_ms: u64,
    /// Maximum delay in milliseconds (caps exponential growth).
    pub max_backoff_ms: u64,
    /// Jitter factor in [0.0, 1.0]. 0.0 = no jitter, 1.0 = full jitter.
    pub jitter: f64,
}

impl RetryPolicy {
    /// A sensible default: 3 attempts, 100ms..10s backoff, 0.3 jitter.
    pub fn default_policy() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff_ms: 100,
            max_backoff_ms: 10_000,
            jitter: 0.3,
        }
    }

    /// Policy that never retries (exactly one attempt).
    pub fn no_retry() -> Self {
        Self {
            max_attempts: 1,
            initial_backoff_ms: 0,
            max_backoff_ms: 0,
            jitter: 0.0,
        }
    }
}
