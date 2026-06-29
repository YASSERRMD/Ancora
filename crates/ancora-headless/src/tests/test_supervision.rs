use crate::supervision::{ExitReason, RestartStrategy, Supervisor, SupervisorConfig};
use std::time::Duration;

#[test]
fn test_service_restarts_on_crash() {
    let mut sup = Supervisor::default();
    sup.on_start();
    let delay = sup.on_exit(ExitReason::Error(1));
    assert!(delay.is_some(), "should restart after error exit");
}

#[test]
fn test_service_does_not_restart_on_clean_exit() {
    let mut sup = Supervisor::default();
    sup.on_start();
    let delay = sup.on_exit(ExitReason::Clean);
    assert!(delay.is_none(), "should not restart after clean exit");
}

#[test]
fn test_restart_limit_exceeded() {
    let config = SupervisorConfig {
        max_restarts: 2,
        strategy: RestartStrategy::Immediate,
        // Use a large stability window so the counter is never reset
        stability_window: Duration::from_secs(3600),
    };
    let mut sup = Supervisor::new(config);
    for _ in 0..2 {
        sup.on_start();
        sup.on_exit(ExitReason::Signal(9));
    }
    // Third crash should hit the limit
    sup.on_start();
    let delay = sup.on_exit(ExitReason::Signal(9));
    assert!(delay.is_none(), "should stop after restart limit");
    assert!(sup.at_restart_limit());
}

#[test]
fn test_exponential_backoff_increases() {
    let config = SupervisorConfig {
        strategy: RestartStrategy::ExponentialBackoff {
            initial_delay_ms: 100,
            max_delay_ms: 10_000,
        },
        max_restarts: 20,
        stability_window: Duration::from_secs(0),
    };
    let mut sup = Supervisor::new(config);
    let mut delays = Vec::new();
    for _ in 0..4 {
        sup.on_start();
        if let Some(d) = sup.on_exit(ExitReason::Error(1)) {
            delays.push(d);
        }
    }
    assert!(delays.len() >= 2);
    // Delays should be non-decreasing
    for i in 1..delays.len() {
        assert!(delays[i] >= delays[i - 1]);
    }
}

#[test]
fn test_restart_strategy_display() {
    assert_eq!(RestartStrategy::Never.to_string(), "never");
    assert_eq!(RestartStrategy::Immediate.to_string(), "immediate");
}

#[test]
fn test_exit_reason_display() {
    assert_eq!(ExitReason::Clean.to_string(), "clean");
    assert_eq!(ExitReason::OomKilled.to_string(), "oom-killed");
}
