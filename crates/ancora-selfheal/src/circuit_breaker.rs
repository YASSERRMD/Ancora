/// Half-open state allows a single probe request through.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CBState {
    Closed,
    Open { until: u64 },
    HalfOpen,
}

pub struct CircuitBreaker {
    pub name: String,
    pub failure_threshold: u32,
    pub reset_timeout_secs: u64,
    consecutive_failures: u32,
    state: CBState,
}

impl CircuitBreaker {
    pub fn new(name: &str, failure_threshold: u32, reset_timeout_secs: u64) -> Self {
        Self {
            name: name.to_string(),
            failure_threshold,
            reset_timeout_secs,
            consecutive_failures: 0,
            state: CBState::Closed,
        }
    }

    pub fn state(&self) -> &CBState {
        &self.state
    }

    pub fn is_open(&self, now: u64) -> bool {
        match &self.state {
            CBState::Open { until } => now < *until,
            _ => false,
        }
    }

    pub fn on_success(&mut self) {
        self.consecutive_failures = 0;
        self.state = CBState::Closed;
    }

    pub fn on_failure(&mut self, now: u64) {
        self.consecutive_failures += 1;
        if self.consecutive_failures >= self.failure_threshold {
            self.state = CBState::Open {
                until: now + self.reset_timeout_secs,
            };
        }
    }

    pub fn try_half_open(&mut self, now: u64) -> bool {
        if let CBState::Open { until } = &self.state {
            if now >= *until {
                self.state = CBState::HalfOpen;
                return true;
            }
        }
        false
    }

    pub fn failure_count(&self) -> u32 {
        self.consecutive_failures
    }
}
