/// Service supervision and automatic restart for the headless agent.
///
/// Implements a supervisor that monitors the agent process, detects
/// crashes, enforces restart back-off, and limits total restart attempts.

use std::time::{Duration, Instant};

/// Restart strategy for the supervisor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartStrategy {
    /// Never restart.
    Never,
    /// Always restart immediately.
    Immediate,
    /// Restart with exponential back-off starting at `initial_delay`.
    ExponentialBackoff { initial_delay_ms: u64, max_delay_ms: u64 },
    /// Restart with a fixed delay between attempts.
    FixedDelay { delay_ms: u64 },
}

impl std::fmt::Display for RestartStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestartStrategy::Never => write!(f, "never"),
            RestartStrategy::Immediate => write!(f, "immediate"),
            RestartStrategy::ExponentialBackoff { initial_delay_ms, max_delay_ms } => {
                write!(f, "exponential-backoff({}ms..{}ms)", initial_delay_ms, max_delay_ms)
            }
            RestartStrategy::FixedDelay { delay_ms } => write!(f, "fixed-delay({}ms)", delay_ms),
        }
    }
}

/// Configuration for the supervisor.
#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    pub strategy: RestartStrategy,
    pub max_restarts: u32,
    /// If the process lives longer than this, reset the restart counter.
    pub stability_window: Duration,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        SupervisorConfig {
            strategy: RestartStrategy::ExponentialBackoff {
                initial_delay_ms: 500,
                max_delay_ms: 30_000,
            },
            max_restarts: 10,
            stability_window: Duration::from_secs(60),
        }
    }
}

/// The reason a supervised process exited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitReason {
    /// Process exited with a zero status (clean shutdown).
    Clean,
    /// Process exited with a non-zero status.
    Error(i32),
    /// Process was killed by a signal.
    Signal(i32),
    /// Process was killed by the OOM killer.
    OomKilled,
    /// Restart limit exceeded.
    RestartLimitExceeded,
}

impl std::fmt::Display for ExitReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExitReason::Clean => write!(f, "clean"),
            ExitReason::Error(code) => write!(f, "error({})", code),
            ExitReason::Signal(sig) => write!(f, "signal({})", sig),
            ExitReason::OomKilled => write!(f, "oom-killed"),
            ExitReason::RestartLimitExceeded => write!(f, "restart-limit-exceeded"),
        }
    }
}

/// A single supervisor event record.
#[derive(Debug, Clone)]
pub struct SupervisorEvent {
    pub restart_count: u32,
    pub exit_reason: ExitReason,
    pub delay: Duration,
}

/// The supervisor state machine.
pub struct Supervisor {
    pub config: SupervisorConfig,
    restart_count: u32,
    last_start: Option<Instant>,
    events: Vec<SupervisorEvent>,
}

impl Supervisor {
    pub fn new(config: SupervisorConfig) -> Self {
        Supervisor {
            config,
            restart_count: 0,
            last_start: None,
            events: Vec::new(),
        }
    }

    /// Called when the supervised process starts.
    pub fn on_start(&mut self) {
        self.last_start = Some(Instant::now());
    }

    /// Called when the supervised process exits.
    /// Returns the delay before the next restart, or None if should not restart.
    pub fn on_exit(&mut self, reason: ExitReason) -> Option<Duration> {
        // Reset counter if process was stable for long enough.
        if let Some(start) = self.last_start {
            if start.elapsed() >= self.config.stability_window {
                self.restart_count = 0;
            }
        }

        if matches!(reason, ExitReason::Clean) {
            self.events.push(SupervisorEvent {
                restart_count: self.restart_count,
                exit_reason: reason,
                delay: Duration::ZERO,
            });
            return None;
        }

        if matches!(self.config.strategy, RestartStrategy::Never) {
            self.events.push(SupervisorEvent {
                restart_count: self.restart_count,
                exit_reason: reason,
                delay: Duration::ZERO,
            });
            return None;
        }

        if self.restart_count >= self.config.max_restarts {
            self.events.push(SupervisorEvent {
                restart_count: self.restart_count,
                exit_reason: ExitReason::RestartLimitExceeded,
                delay: Duration::ZERO,
            });
            return None;
        }

        let delay = self.compute_delay();
        self.restart_count += 1;
        self.events.push(SupervisorEvent {
            restart_count: self.restart_count,
            exit_reason: reason,
            delay,
        });
        Some(delay)
    }

    fn compute_delay(&self) -> Duration {
        match &self.config.strategy {
            RestartStrategy::Immediate => Duration::ZERO,
            RestartStrategy::FixedDelay { delay_ms } => Duration::from_millis(*delay_ms),
            RestartStrategy::ExponentialBackoff { initial_delay_ms, max_delay_ms } => {
                let exp = 1u64 << self.restart_count.min(20);
                let ms = (initial_delay_ms * exp).min(*max_delay_ms);
                Duration::from_millis(ms)
            }
            RestartStrategy::Never => Duration::ZERO,
        }
    }

    pub fn restart_count(&self) -> u32 {
        self.restart_count
    }

    pub fn events(&self) -> &[SupervisorEvent] {
        &self.events
    }

    pub fn at_restart_limit(&self) -> bool {
        self.restart_count >= self.config.max_restarts
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new(SupervisorConfig::default())
    }
}
