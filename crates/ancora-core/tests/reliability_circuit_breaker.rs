// Reliability: circuit breaker -- open after threshold, half-open probe, close.

#[derive(Debug, PartialEq, Clone)]
enum CBState {
    Closed,
    Open,
    HalfOpen,
}

struct CircuitBreaker {
    state: CBState,
    failures: u32,
    successes_in_half_open: u32,
    threshold: u32,
    success_threshold: u32,
}

impl CircuitBreaker {
    fn new(threshold: u32, success_threshold: u32) -> Self {
        Self { state: CBState::Closed, failures: 0, successes_in_half_open: 0, threshold, success_threshold }
    }

    fn call<F: Fn() -> Result<(), ()>>(&mut self, f: F) -> Result<(), ()> {
        match self.state {
            CBState::Open => Err(()),
            CBState::Closed | CBState::HalfOpen => {
                match f() {
                    Ok(()) => {
                        if self.state == CBState::HalfOpen {
                            self.successes_in_half_open += 1;
                            if self.successes_in_half_open >= self.success_threshold {
                                self.state = CBState::Closed;
                                self.failures = 0;
                                self.successes_in_half_open = 0;
                            }
                        } else {
                            self.failures = 0;
                        }
                        Ok(())
                    }
                    Err(()) => {
                        self.failures += 1;
                        if self.failures >= self.threshold { self.state = CBState::Open; }
                        Err(())
                    }
                }
            }
        }
    }

    fn probe(&mut self) { if self.state == CBState::Open { self.state = CBState::HalfOpen; } }
}

#[test]
fn test_opens_after_threshold_failures() {
    let mut cb = CircuitBreaker::new(3, 1);
    for _ in 0..3 { let _ = cb.call(|| Err(())); }
    assert_eq!(cb.state, CBState::Open);
}

#[test]
fn test_open_rejects_calls() {
    let mut cb = CircuitBreaker::new(2, 1);
    for _ in 0..2 { let _ = cb.call(|| Err(())); }
    let r = cb.call(|| Ok(()));
    assert!(r.is_err());
}

#[test]
fn test_probe_transitions_to_half_open() {
    let mut cb = CircuitBreaker::new(2, 1);
    for _ in 0..2 { let _ = cb.call(|| Err(())); }
    cb.probe();
    assert_eq!(cb.state, CBState::HalfOpen);
}

#[test]
fn test_half_open_success_closes() {
    let mut cb = CircuitBreaker::new(2, 1);
    for _ in 0..2 { let _ = cb.call(|| Err(())); }
    cb.probe();
    cb.call(|| Ok(())).unwrap();
    assert_eq!(cb.state, CBState::Closed);
}

#[test]
fn test_half_open_failure_reopens() {
    let mut cb = CircuitBreaker::new(2, 1);
    for _ in 0..2 { let _ = cb.call(|| Err(())); }
    cb.probe();
    let _ = cb.call(|| Err(()));
    assert_eq!(cb.state, CBState::Open);
}

#[test]
fn test_closed_on_success_resets_failure_count() {
    let mut cb = CircuitBreaker::new(3, 1);
    let _ = cb.call(|| Err(()));
    let _ = cb.call(|| Err(()));
    cb.call(|| Ok(())).unwrap();
    assert_eq!(cb.failures, 0);
    assert_eq!(cb.state, CBState::Closed);
}
