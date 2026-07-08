// Staged rollout to fleet

use crate::registration::DeviceId;
use std::collections::HashMap;

/// Rollout phase definition
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RolloutPhase {
    pub name: String,
    /// Percentage of the total fleet targeted in this phase (0–100)
    pub target_percent: u8,
    pub device_ids: Vec<DeviceId>,
}

impl RolloutPhase {
    pub fn new(name: impl Into<String>, target_percent: u8) -> Self {
        Self {
            name: name.into(),
            target_percent,
            device_ids: Vec::new(),
        }
    }

    pub fn add_device(&mut self, id: DeviceId) {
        self.device_ids.push(id);
    }
}

/// Status of a rollout for a device
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RolloutStatus {
    Pending,
    InProgress,
    Complete,
    RolledBack,
    Failed(String),
}

/// A staged rollout plan
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RolloutPlan {
    pub rollout_id: String,
    pub artifact_id: String,
    pub phases: Vec<RolloutPhase>,
}

impl RolloutPlan {
    pub fn new(rollout_id: impl Into<String>, artifact_id: impl Into<String>) -> Self {
        Self {
            rollout_id: rollout_id.into(),
            artifact_id: artifact_id.into(),
            phases: Vec::new(),
        }
    }

    pub fn add_phase(&mut self, phase: RolloutPhase) {
        self.phases.push(phase);
    }

    pub fn total_devices(&self) -> usize {
        self.phases.iter().map(|p| p.device_ids.len()).sum()
    }
}

/// Rollout engine — executes and tracks staged rollouts
#[derive(Debug, Default)]
pub struct RolloutEngine {
    /// (rollout_id, device_id) -> RolloutStatus
    statuses: HashMap<(String, String), RolloutStatus>,
}

impl RolloutEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a single phase of the rollout plan
    pub fn execute_phase(&mut self, plan: &RolloutPlan, phase_index: usize) -> usize {
        let phase = match plan.phases.get(phase_index) {
            Some(p) => p,
            None => return 0,
        };

        let mut executed = 0;
        for device_id in &phase.device_ids {
            let key = (plan.rollout_id.clone(), device_id.0.clone());
            self.statuses.insert(key, RolloutStatus::Complete);
            executed += 1;
        }
        executed
    }

    /// Execute all phases sequentially
    pub fn execute_all(&mut self, plan: &RolloutPlan) -> usize {
        let count = plan.phases.len();
        let mut total = 0;
        for i in 0..count {
            total += self.execute_phase(plan, i);
        }
        total
    }

    pub fn device_status(&self, rollout_id: &str, device_id: &DeviceId) -> Option<&RolloutStatus> {
        let key = (rollout_id.to_string(), device_id.0.clone());
        self.statuses.get(&key)
    }

    pub fn completed_devices(&self, rollout_id: &str) -> Vec<String> {
        self.statuses
            .iter()
            .filter(|((rid, _), status)| rid == rollout_id && **status == RolloutStatus::Complete)
            .map(|((_, did), _)| did.clone())
            .collect()
    }

    /// Roll back a device from a rollout
    pub fn rollback(&mut self, rollout_id: &str, device_id: &DeviceId) {
        let key = (rollout_id.to_string(), device_id.0.clone());
        self.statuses.insert(key, RolloutStatus::RolledBack);
    }
}
