use crate::decision::ScaleDecision;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Scaling signal emitted to observability and external systems (e.g. HPA).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSignal {
    pub ts: DateTime<Utc>,
    pub decision: ScaleDecision,
    pub current_workers: usize,
    pub desired_workers: usize,
    pub queue_depth: usize,
    pub utilization: f64,
}

impl ScaleSignal {
    pub fn from_decision(
        decision: ScaleDecision,
        current: usize,
        queue_depth: usize,
        utilization: f64,
    ) -> Self {
        let desired = match &decision {
            ScaleDecision::ScaleUp { by } => current + by,
            ScaleDecision::ScaleDown { by } => current.saturating_sub(*by),
            ScaleDecision::NoOp { .. } => current,
        };
        ScaleSignal {
            ts: Utc::now(),
            decision,
            current_workers: current,
            desired_workers: desired,
            queue_depth,
            utilization,
        }
    }
}
