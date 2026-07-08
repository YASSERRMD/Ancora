/// Engine health and readiness checking.
///
/// Provides a unified `HealthStatus` type and a `HealthChecker` trait
/// so any engine can be checked for liveness and readiness without
/// coupling to a specific transport implementation.
use crate::model::EngineKind;
use std::time::{Duration, Instant};

/// Liveness / readiness state of an engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthState {
    /// Engine is reachable and accepting requests.
    Healthy,
    /// Engine is reachable but not yet ready (e.g., model loading).
    Starting,
    /// Engine is unreachable or returning errors.
    Unhealthy,
    /// Health check has never been run.
    Unknown,
}

impl std::fmt::Display for HealthState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HealthState::Healthy => "healthy",
            HealthState::Starting => "starting",
            HealthState::Unhealthy => "unhealthy",
            HealthState::Unknown => "unknown",
        };
        write!(f, "{}", s)
    }
}

/// Detailed health status for an engine.
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub engine: EngineKind,
    pub state: HealthState,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
    pub checked_at: Option<Instant>,
}

impl HealthStatus {
    pub fn healthy(engine: EngineKind, latency_ms: u64) -> Self {
        HealthStatus {
            engine,
            state: HealthState::Healthy,
            message: None,
            latency_ms: Some(latency_ms),
            checked_at: Some(Instant::now()),
        }
    }

    pub fn unhealthy(engine: EngineKind, reason: &str) -> Self {
        HealthStatus {
            engine,
            state: HealthState::Unhealthy,
            message: Some(reason.to_string()),
            latency_ms: None,
            checked_at: Some(Instant::now()),
        }
    }

    pub fn starting(engine: EngineKind) -> Self {
        HealthStatus {
            engine,
            state: HealthState::Starting,
            message: Some("model loading".to_string()),
            latency_ms: None,
            checked_at: Some(Instant::now()),
        }
    }

    pub fn unknown(engine: EngineKind) -> Self {
        HealthStatus {
            engine,
            state: HealthState::Unknown,
            message: None,
            latency_ms: None,
            checked_at: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == HealthState::Healthy
    }
}

/// Trait implemented by each engine-specific health prober.
pub trait HealthChecker {
    fn check(&self) -> HealthStatus;
}

/// A mock health checker for offline tests.
pub struct MockHealthChecker {
    engine: EngineKind,
    state: HealthState,
    latency_ms: u64,
}

impl MockHealthChecker {
    pub fn healthy(engine: EngineKind) -> Self {
        MockHealthChecker {
            engine,
            state: HealthState::Healthy,
            latency_ms: 2,
        }
    }

    pub fn unhealthy(engine: EngineKind) -> Self {
        MockHealthChecker {
            engine,
            state: HealthState::Unhealthy,
            latency_ms: 0,
        }
    }

    pub fn starting(engine: EngineKind) -> Self {
        MockHealthChecker {
            engine,
            state: HealthState::Starting,
            latency_ms: 0,
        }
    }
}

impl HealthChecker for MockHealthChecker {
    fn check(&self) -> HealthStatus {
        match &self.state {
            HealthState::Healthy => HealthStatus::healthy(self.engine.clone(), self.latency_ms),
            HealthState::Unhealthy => {
                HealthStatus::unhealthy(self.engine.clone(), "mock: unreachable")
            }
            HealthState::Starting => HealthStatus::starting(self.engine.clone()),
            HealthState::Unknown => HealthStatus::unknown(self.engine.clone()),
        }
    }
}

/// Poll a health checker until ready or max duration exceeded.
pub fn wait_ready<C: HealthChecker>(
    checker: &C,
    timeout: Duration,
    interval: Duration,
) -> HealthStatus {
    let start = Instant::now();
    loop {
        let status = checker.check();
        if status.is_ready() || start.elapsed() >= timeout {
            return status;
        }
        // In a real system we would sleep here; for tests the mock resolves instantly.
        if interval.is_zero() {
            return status;
        }
        std::thread::sleep(interval);
    }
}
