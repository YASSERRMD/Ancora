use crate::error::DeployError;
use crate::worker::VersionedWorker;

pub struct CanaryController {
    pub stable: Vec<VersionedWorker>,
    pub canary: Vec<VersionedWorker>,
    /// Fraction of traffic routed to canary (0.0..=1.0).
    pub canary_pct: f64,
    /// Error rate threshold above which the health gate fails.
    pub error_threshold_pct: f64,
    /// Accumulated error count and total count for the canary subset.
    pub canary_errors: u64,
    pub canary_total: u64,
}

impl CanaryController {
    pub fn new(
        stable: Vec<VersionedWorker>,
        canary: Vec<VersionedWorker>,
        canary_pct: f64,
        error_threshold_pct: f64,
    ) -> Self {
        Self {
            stable,
            canary,
            canary_pct,
            error_threshold_pct,
            canary_errors: 0,
            canary_total: 0,
        }
    }

    /// Route a request: returns `true` if it should go to canary.
    pub fn route_to_canary(&self, request_index: u64) -> bool {
        // Deterministic: send to canary every 1-in-(1/pct) requests
        if self.canary_pct <= 0.0 {
            return false;
        }
        let period = (1.0 / self.canary_pct).round() as u64;
        period > 0 && request_index.is_multiple_of(period)
    }

    /// Record a canary result.
    pub fn record_canary_result(&mut self, error: bool) {
        self.canary_total += 1;
        if error {
            self.canary_errors += 1;
        }
    }

    /// Check health gate; triggers rollback if error rate exceeds threshold.
    pub fn check_health_gate(&self) -> Result<(), DeployError> {
        if self.canary_total == 0 {
            return Ok(());
        }
        let rate = (self.canary_errors as f64 / self.canary_total as f64) * 100.0;
        if rate > self.error_threshold_pct {
            Err(DeployError::CanaryHealthGateFailed {
                error_rate: rate,
                threshold: self.error_threshold_pct,
            })
        } else {
            Ok(())
        }
    }

    /// Promote canary to stable (canary becomes the new stable pool).
    pub fn promote(&mut self) {
        self.stable = self.canary.clone();
        self.canary = vec![];
        self.canary_pct = 0.0;
    }

    /// Rollback: zero out canary and reset traffic percentage.
    pub fn rollback(&mut self) {
        self.canary = vec![];
        self.canary_pct = 0.0;
        self.canary_errors = 0;
        self.canary_total = 0;
    }
}
