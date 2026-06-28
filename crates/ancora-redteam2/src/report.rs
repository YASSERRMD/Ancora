use crate::attack::AttackLog;
use crate::audit::RedTeamAuditLog;
use crate::detection::DetectionLog;
use crate::objective::ObjectiveTracker;
use crate::store::ScenarioStore;

pub struct RedTeamReport {
    pub total_scenarios: usize,
    pub active_scenarios: usize,
    pub total_attack_steps: usize,
    pub successful_steps: usize,
    pub total_detections: usize,
    pub true_positives: usize,
    pub total_objectives: usize,
    pub achieved_objectives: usize,
    pub total_audit_entries: usize,
    pub tick: u64,
}

impl RedTeamReport {
    pub fn generate(
        store: &ScenarioStore,
        attacks: &AttackLog,
        detections: &DetectionLog,
        objectives: &ObjectiveTracker,
        audit: &RedTeamAuditLog,
        tick: u64,
    ) -> Self {
        Self {
            total_scenarios: store.count(),
            active_scenarios: store.active().len(),
            total_attack_steps: attacks.count(),
            successful_steps: attacks.successful().len(),
            total_detections: detections.count(),
            true_positives: detections.true_positives().len(),
            total_objectives: objectives.count(),
            achieved_objectives: objectives.achieved_count(),
            total_audit_entries: audit.count(),
            tick,
        }
    }

    pub fn objective_progress(&self) -> f64 {
        if self.total_objectives == 0 { return 0.0; }
        self.achieved_objectives as f64 / self.total_objectives as f64
    }
}
