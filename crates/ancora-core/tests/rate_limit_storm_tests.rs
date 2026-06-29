/// Rate-limit storm reliability tests (offline).
///
/// Simulates a burst of ModelHttp 429 errors followed by recovery, and
/// validates that the retry engine handles the storm correctly: applying
/// exponential back-off, not exceeding the attempt budget, and ultimately
/// succeeding or exhausting gracefully.
use ancora_core::{
    error::AncoraError,
    retry::{classify, run_with_retry, ErrorClass, RetryOutcome, RetryPolicy},
};

fn rate_limit_err() -> AncoraError {
    AncoraError::ModelHttp { status: 429, body: "rate limited".into() }
}

fn overload_err() -> AncoraError {
    AncoraError::ModelHttp { status: 503, body: "service overloaded".into() }
}

fn aggressive_policy(max: u32) -> RetryPolicy {
    RetryPolicy {
        max_attempts: max,
        initial_backoff_ms: 0,
        max_backoff_ms: 0,
        jitter: 0.0,
    }
}

#[test]
fn rate_limit_429_is_classified_retryable() {
    let err = rate_limit_err();
    assert_eq!(classify(&err), ErrorClass::Retryable);
}

#[test]
fn service_overload_503_is_classified_retryable() {
    let err = overload_err();
    assert_eq!(classify(&err), ErrorClass::Retryable);
}

#[test]
fn storm_of_three_429s_then_success_completes_on_fourth_attempt() {
    let policy = aggressive_policy(5);
    let mut calls = 0u32;

    let outcome = run_with_retry(
        &policy,
        |_| {
            calls += 1;
            if calls <= 3 {
                Err(rate_limit_err())
            } else {
                Ok("success")
            }
        },
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 4, .. }));
}

#[test]
fn storm_exhausts_budget_when_never_recovering() {
    let policy = aggressive_policy(4);

    let outcome = run_with_retry(
        &policy,
        |_| Err::<(), _>(rate_limit_err()),
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Exhausted { attempts: 4, .. }));
}

#[test]
fn sleep_called_between_each_retry_in_a_storm() {
    let policy = aggressive_policy(4);
    let mut sleeps = 0u32;

    run_with_retry(
        &policy,
        |_| Err::<(), _>(rate_limit_err()),
        |_| sleeps += 1,
    );

    assert_eq!(sleeps, 3, "sleep between attempts 1-2, 2-3, and 3-4");
}

#[test]
fn one_429_then_503_then_success_uses_three_attempts() {
    let policy = aggressive_policy(5);
    let mut calls = 0u32;

    let outcome = run_with_retry(
        &policy,
        |_| {
            calls += 1;
            match calls {
                1 => Err(rate_limit_err()),
                2 => Err(overload_err()),
                _ => Ok("recovered"),
            }
        },
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Ok { attempts: 3, .. }));
}

#[test]
fn terminal_error_interrupts_storm_immediately() {
    let policy = aggressive_policy(10);
    let mut calls = 0u32;

    let outcome: RetryOutcome<()> = run_with_retry(
        &policy,
        |_| {
            calls += 1;
            if calls == 1 {
                Err(rate_limit_err())
            } else {
                Err(AncoraError::PolicyResidency("blocked".into()))
            }
        },
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Terminal { .. }));
    assert_eq!(calls, 2, "must stop immediately after terminal error");
}

#[test]
fn max_attempts_one_means_no_retry_on_429() {
    let policy = aggressive_policy(1);

    let outcome = run_with_retry(
        &policy,
        |_| Err::<(), _>(rate_limit_err()),
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Exhausted { attempts: 1, .. }));
}

#[test]
fn storm_of_ten_429s_exhausts_budget_of_ten() {
    let policy = aggressive_policy(10);
    let mut count = 0u32;

    let outcome = run_with_retry(
        &policy,
        |_| { count += 1; Err::<(), _>(rate_limit_err()) },
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Exhausted { attempts: 10, .. }));
    assert_eq!(count, 10);
}
