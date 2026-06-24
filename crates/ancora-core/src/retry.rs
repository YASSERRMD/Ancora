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

/// Outcome returned by `run_with_retry`.
pub enum RetryOutcome<T> {
    /// The operation succeeded on attempt `attempt` (1-indexed).
    Ok { value: T, attempts: u32 },
    /// All attempts exhausted; the last error is returned.
    Exhausted { error: AncoraError, attempts: u32 },
    /// A terminal error stopped retrying early.
    Terminal { error: AncoraError, attempt: u32 },
}

/// Execute `op` up to `policy.max_attempts` times, sleeping between attempts.
///
/// The delay for attempt `n` (0-indexed) is:
///   delay = min(initial_backoff_ms * 2^n, max_backoff_ms)
///   delay = delay * (1 - jitter * jitter_factor)
///
/// The `sleep_fn` parameter replaces actual sleep so tests run without delay.
/// Pass `|_ms| {}` to skip sleeping entirely.
pub fn run_with_retry<T, F, S>(
    policy: &RetryPolicy,
    mut op: F,
    mut sleep_fn: S,
) -> RetryOutcome<T>
where
    F: FnMut(u32) -> Result<T, AncoraError>,
    S: FnMut(u64),
{
    for attempt in 1..=policy.max_attempts {
        match op(attempt) {
            Ok(value) => return RetryOutcome::Ok { value, attempts: attempt },
            Err(err) => {
                if classify(&err) == ErrorClass::Terminal {
                    return RetryOutcome::Terminal { error: err, attempt };
                }
                if attempt < policy.max_attempts {
                    let exp = (attempt - 1) as u32;
                    let shift = exp.min(63);
                    let base = policy
                        .initial_backoff_ms
                        .saturating_mul(1u64 << shift);
                    let capped = base.min(policy.max_backoff_ms);
                    let jittered = (capped as f64 * (1.0 - policy.jitter * 0.5)) as u64;
                    sleep_fn(jittered);
                } else {
                    return RetryOutcome::Exhausted { error: err, attempts: attempt };
                }
            }
        }
    }
    unreachable!("loop always returns")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn retryable() -> AncoraError {
        AncoraError::ModelHttp { status: 500, body: "err".to_string() }
    }

    fn terminal() -> AncoraError {
        AncoraError::PolicyPermission("denied".to_string())
    }

    #[test]
    fn retries_stop_at_max_attempts() {
        let policy = RetryPolicy { max_attempts: 3, initial_backoff_ms: 0, max_backoff_ms: 0, jitter: 0.0 };
        let mut call_count = 0u32;

        let outcome = run_with_retry(
            &policy,
            |_attempt| {
                call_count += 1;
                Err::<(), _>(retryable())
            },
            |_ms| {},
        );

        assert_eq!(call_count, 3, "must attempt exactly max_attempts times");
        assert!(matches!(outcome, RetryOutcome::Exhausted { attempts: 3, .. }));
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
