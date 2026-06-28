use serde::{Deserialize, Serialize};

/// SLO targets for availability and latency.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SloTarget {
    pub name: String,
    /// Target availability as a fraction (e.g. 0.999 = 99.9%).
    pub availability_target: f64,
    /// P99 latency target in ms.
    pub latency_p99_ms: u64,
    /// Window over which SLO is evaluated in seconds.
    pub window_secs: u64,
}

/// Tracks the current error budget for an SLO.
#[derive(Debug)]
pub struct ErrorBudget {
    pub target: SloTarget,
    /// Observed availability (0.0-1.0).
    pub observed_availability: f64,
    /// Total request count in current window.
    pub total_requests: u64,
    /// Failed request count in current window.
    pub failed_requests: u64,
}

impl ErrorBudget {
    pub fn new(target: SloTarget) -> Self {
        Self { target, observed_availability: 1.0, total_requests: 0, failed_requests: 0 }
    }

    pub fn record(&mut self, success: bool) {
        self.total_requests += 1;
        if !success {
            self.failed_requests += 1;
        }
        self.observed_availability = if self.total_requests == 0 {
            1.0
        } else {
            (self.total_requests - self.failed_requests) as f64 / self.total_requests as f64
        };
    }

    /// Remaining error budget as fraction of allowed errors consumed.
    /// 1.0 = full budget remaining; 0.0 = budget exhausted.
    pub fn budget_remaining_fraction(&self) -> f64 {
        let allowed_error_rate = 1.0 - self.target.availability_target;
        if allowed_error_rate <= 0.0 {
            return 0.0;
        }
        let observed_error_rate = 1.0 - self.observed_availability;
        let consumed = observed_error_rate / allowed_error_rate;
        (1.0 - consumed).max(0.0)
    }

    pub fn is_breached(&self) -> bool {
        self.observed_availability < self.target.availability_target
    }
}

/// Burn-rate alert: fires when the error budget is burning faster than sustainable.
#[derive(Debug)]
pub struct BurnRateAlert {
    /// Multiplier above sustainable burn rate that triggers the alert.
    pub threshold_multiplier: f64,
    pub fired: bool,
    pub current_burn_rate: f64,
}

impl BurnRateAlert {
    pub fn new(threshold_multiplier: f64) -> Self {
        Self { threshold_multiplier, fired: false, current_burn_rate: 0.0 }
    }

    /// Evaluate burn rate given the observed error rate and the SLO target.
    pub fn evaluate(&mut self, observed_error_rate: f64, slo_error_rate: f64) {
        let sustainable_burn = 1.0;
        if slo_error_rate <= 0.0 {
            self.fired = false;
            return;
        }
        self.current_burn_rate = observed_error_rate / slo_error_rate;
        self.fired = self.current_burn_rate > sustainable_burn * self.threshold_multiplier;
    }
}

/// Default burn-rate multipliers used in operational deployments.
pub const FAST_BURN_MULTIPLIER: f64 = 14.4;
pub const SLOW_BURN_MULTIPLIER: f64 = 3.0;
