// Chaos: retry with exponential back-off (offline simulation).

fn backoff_ms(attempt: u32, base_ms: u64) -> u64 {
    base_ms * (1u64 << attempt.min(10))
}

struct RetryOutcome {
    attempts: u32,
    final_ms: u64,
    succeeded: bool,
}

fn simulate_retry(fail_for: u32, max_attempts: u32, base_ms: u64) -> RetryOutcome {
    let mut total = 0u64;
    for attempt in 0..max_attempts {
        let delay = backoff_ms(attempt, base_ms);
        total += delay;
        if attempt >= fail_for {
            return RetryOutcome {
                attempts: attempt + 1,
                final_ms: total,
                succeeded: true,
            };
        }
    }
    RetryOutcome {
        attempts: max_attempts,
        final_ms: total,
        succeeded: false,
    }
}

#[test]
fn test_retry_succeeds_on_third_attempt() {
    let r = simulate_retry(2, 5, 50);
    assert!(r.succeeded);
    assert_eq!(r.attempts, 3);
}

#[test]
fn test_retry_exhausts_max_attempts() {
    let r = simulate_retry(10, 4, 50);
    assert!(!r.succeeded);
    assert_eq!(r.attempts, 4);
}

#[test]
fn test_backoff_doubles_each_attempt() {
    assert_eq!(backoff_ms(0, 100), 100);
    assert_eq!(backoff_ms(1, 100), 200);
    assert_eq!(backoff_ms(2, 100), 400);
    assert_eq!(backoff_ms(3, 100), 800);
}

#[test]
fn test_backoff_caps_at_shift_10() {
    assert_eq!(backoff_ms(10, 1), 1024);
    assert_eq!(backoff_ms(11, 1), 1024);
    assert_eq!(backoff_ms(20, 1), 1024);
}

#[test]
fn test_total_delay_increases_with_attempts() {
    let r2 = simulate_retry(99, 2, 10);
    let r4 = simulate_retry(99, 4, 10);
    assert!(r4.final_ms > r2.final_ms);
}

#[test]
fn test_immediate_success_on_first_attempt() {
    let r = simulate_retry(0, 3, 50);
    assert!(r.succeeded);
    assert_eq!(r.attempts, 1);
}
