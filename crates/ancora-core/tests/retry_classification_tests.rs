use ancora_core::error::AncoraError;
use ancora_core::retry::{classify, run_with_retry, ErrorClass, RetryOutcome, RetryPolicy};

fn default_policy(max: u32) -> RetryPolicy {
    RetryPolicy {
        max_attempts: max,
        initial_backoff_ms: 0,
        max_backoff_ms: 0,
        jitter: 0.0,
    }
}

// ---- Classification matrix ----

#[test]
fn model_http_error_is_retryable() {
    let err = AncoraError::ModelHttp { status: 503, body: "overloaded".into() };
    assert_eq!(classify(&err), ErrorClass::Retryable);
}

#[test]
fn model_unreachable_is_retryable() {
    let err = AncoraError::ModelUnreachable("connection refused".into());
    assert_eq!(classify(&err), ErrorClass::Retryable);
}

#[test]
fn timeout_is_retryable() {
    let err = AncoraError::Timeout { timeout_ms: 5000 };
    assert_eq!(classify(&err), ErrorClass::Retryable);
}

#[test]
fn storage_error_is_retryable() {
    let err = AncoraError::Storage("disk full".into());
    assert_eq!(classify(&err), ErrorClass::Retryable);
}

#[test]
fn tool_failed_is_retryable() {
    let err = AncoraError::ToolFailed { name: "search".into(), message: "timeout".into() };
    assert_eq!(classify(&err), ErrorClass::Retryable);
}

#[test]
fn policy_residency_is_terminal() {
    let err = AncoraError::PolicyResidency("eu-only".into());
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn max_steps_is_terminal() {
    let err = AncoraError::MaxSteps { max_steps: 10 };
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn nondeterminism_is_terminal() {
    let err = AncoraError::Nondeterminism {
        seq: 3,
        expected: "a".into(),
        got: "b".into(),
    };
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn graph_invalid_is_terminal() {
    let err = AncoraError::GraphInvalid("cycle detected".into());
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn cancelled_is_terminal() {
    let err = AncoraError::Cancelled("user requested".into());
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

// ---- run_with_retry behavior ----

#[test]
fn retry_succeeds_on_first_attempt() {
    let policy = default_policy(3);
    let outcome = run_with_retry(&policy, |_| Ok::<_, AncoraError>("ok"), |_| {});
    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 1, .. }));
}

#[test]
fn retry_succeeds_on_second_attempt_after_retryable_error() {
    let policy = default_policy(3);
    let mut call = 0u32;
    let outcome = run_with_retry(
        &policy,
        |_| {
            call += 1;
            if call == 1 {
                Err(AncoraError::ModelUnreachable("first fail".into()))
            } else {
                Ok("recovered")
            }
        },
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 2, .. }));
}

#[test]
fn terminal_error_stops_immediately() {
    let policy = default_policy(5);
    let mut calls = 0u32;
    let outcome = run_with_retry(
        &policy,
        |_| {
            calls += 1;
            Err::<(), _>(AncoraError::PolicyResidency("blocked".into()))
        },
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Terminal { attempt: 1, .. }));
    assert_eq!(calls, 1, "must stop after first terminal error");
}

#[test]
fn exhausted_after_max_attempts() {
    let policy = default_policy(3);
    let outcome = run_with_retry(
        &policy,
        |_| Err::<(), _>(AncoraError::ModelUnreachable("always fail".into())),
        |_| {},
    );
    assert!(matches!(outcome, RetryOutcome::Exhausted { attempts: 3, .. }));
}

#[test]
fn sleep_fn_called_between_retries() {
    let policy = default_policy(3);
    let mut sleeps = 0u32;
    run_with_retry(
        &policy,
        |_| Err::<(), _>(AncoraError::ModelUnreachable("fail".into())),
        |_| sleeps += 1,
    );
    assert_eq!(sleeps, 2, "sleep must be called between attempts 1-2 and 2-3");
}
