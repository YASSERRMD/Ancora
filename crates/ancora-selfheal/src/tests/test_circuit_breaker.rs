use crate::circuit_breaker::{CBState, CircuitBreaker};

#[test]
fn starts_closed() {
    let cb = CircuitBreaker::new("provider-a", 3, 60);
    assert_eq!(cb.state(), &CBState::Closed);
}

#[test]
fn opens_after_threshold_failures() {
    let mut cb = CircuitBreaker::new("provider-a", 3, 60);
    cb.on_failure(0);
    cb.on_failure(0);
    cb.on_failure(0);
    assert!(cb.is_open(10));
}

#[test]
fn single_success_closes_from_half_open() {
    let mut cb = CircuitBreaker::new("p", 1, 30);
    cb.on_failure(0);
    cb.try_half_open(30);
    cb.on_success();
    assert_eq!(cb.state(), &CBState::Closed);
    assert_eq!(cb.failure_count(), 0);
}

#[test]
fn half_open_transition_after_timeout() {
    let mut cb = CircuitBreaker::new("p", 1, 30);
    cb.on_failure(0);
    assert!(!cb.try_half_open(10));
    assert!(cb.try_half_open(30));
    assert_eq!(cb.state(), &CBState::HalfOpen);
}
