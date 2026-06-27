/// End-to-end error recovery and exponential backoff tests (offline).
///
/// Validates that transient errors trigger appropriate retry behavior, that
/// the backoff parameters are respected, and that the engine converges to
/// a successful outcome after transient failures without exceeding the
/// attempt budget.
use ancora_core::{
    error::AncoraError,
    retry::{classify, run_with_retry, ErrorClass, RetryOutcome, RetryPolicy},
};

fn policy(max: u32, initial_ms: u64, max_ms: u64, jitter: f64) -> RetryPolicy {
    RetryPolicy { max_attempts: max, initial_backoff_ms: initial_ms, max_backoff_ms: max_ms, jitter }
}

fn transient(msg: &str) -> AncoraError {
    AncoraError::ModelUnreachable(msg.into())
}

fn storage_err() -> AncoraError {
    AncoraError::Storage("disk IO error".into())
}

fn timeout_err() -> AncoraError {
    AncoraError::Timeout { timeout_ms: 5000 }
}

#[test]
fn model_unreachable_is_retryable() {
    assert_eq!(classify(&transient("connection reset")), ErrorClass::Retryable);
}

#[test]
fn storage_error_is_retryable() {
    assert_eq!(classify(&storage_err()), ErrorClass::Retryable);
}

#[test]
fn timeout_is_retryable() {
    assert_eq!(classify(&timeout_err()), ErrorClass::Retryable);
}

#[test]
fn immediate_success_uses_one_attempt() {
    let p = policy(5, 0, 0, 0.0);
    let outcome = run_with_retry(&p, |_| Ok::<_, AncoraError>("done"), |_| {});
    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 1, .. }));
}

#[test]
fn one_transient_then_success_uses_two_attempts() {
    let p = policy(5, 0, 0, 0.0);
    let mut n = 0u32;
    let outcome = run_with_retry(
        &p,
        |_| { n += 1; if n == 1 { Err(transient("first fail")) } else { Ok("ok") } },
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 2, .. }));
}

#[test]
fn two_storage_errors_then_success_uses_three_attempts() {
    let p = policy(5, 0, 0, 0.0);
    let mut n = 0u32;
    let outcome = run_with_retry(
        &p,
        |_| { n += 1; if n <= 2 { Err(storage_err()) } else { Ok("ok") } },
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 3, .. }));
}

#[test]
fn alternating_transient_and_storage_errors_both_retry() {
    let p = policy(6, 0, 0, 0.0);
    let mut n = 0u32;
    let outcome = run_with_retry(
        &p,
        |_| {
            n += 1;
            match n {
                1 => Err(transient("transient 1")),
                2 => Err(storage_err()),
                3 => Err(timeout_err()),
                _ => Ok("recovered"),
            }
        },
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 4, .. }));
}

#[test]
fn budget_exhausted_when_all_transient() {
    let p = policy(3, 0, 0, 0.0);
    let outcome = run_with_retry(
        &p,
        |_| Err::<(), _>(transient("always fail")),
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Exhausted { attempts: 3, .. }));
}

#[test]
fn sleep_fn_called_between_every_attempt() {
    let p = policy(4, 0, 0, 0.0);
    let mut sleeps = 0u32;
    run_with_retry(&p, |_| Err::<(), _>(transient("fail")), |_| sleeps += 1);
    assert_eq!(sleeps, 3, "3 sleeps between 4 attempts");
}

#[test]
fn sleep_fn_receives_attempt_number() {
    let p = policy(3, 0, 0, 0.0);
    let mut seen_attempts: Vec<u32> = Vec::new();
    run_with_retry(
        &p,
        |_| Err::<(), _>(transient("fail")),
        |attempt| seen_attempts.push(attempt),
    );
    assert_eq!(seen_attempts, vec![1, 2]);
}

#[test]
fn zero_backoff_policy_never_sleeps_for_real() {
    let p = policy(5, 0, 0, 0.0);
    let start = std::time::Instant::now();
    run_with_retry(&p, |_| Err::<(), _>(transient("fail")), |_| {});
    assert!(start.elapsed().as_millis() < 500, "zero backoff must complete fast");
}

#[test]
fn mixed_error_types_all_retry_until_exhausted() {
    let errors: Vec<AncoraError> = vec![
        transient("a"),
        storage_err(),
        timeout_err(),
        AncoraError::ModelHttp { status: 503, body: String::new() },
    ];
    let p = policy(4, 0, 0, 0.0);
    let mut idx = 0usize;
    let outcome = run_with_retry(
        &p,
        |_| { let e = errors[idx.min(errors.len() - 1)].clone(); idx += 1; Err::<(), _>(e) },
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Exhausted { attempts: 4, .. }));
}
