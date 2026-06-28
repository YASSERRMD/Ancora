use crate::attack::AttackLog;
use crate::detection::DetectionLog;
use crate::scenario::RedTeamScenario;

pub struct RedTeamStats {
    pub total_scenarios: usize,
    pub completed_scenarios: usize,
    pub total_attack_steps: usize,
    pub successful_steps: usize,
    pub detected_steps: usize,
    pub total_detections: usize,
    pub true_positive_detections: usize,
    pub detection_rate: f64,
    pub success_rate: f64,
}

impl RedTeamStats {
    pub fn compute(
        scenarios: &[&RedTeamScenario],
        attacks: &AttackLog,
        detections: &DetectionLog,
    ) -> Self {
        let total_scenarios = scenarios.len();
        let completed_scenarios = scenarios.iter().filter(|s| s.completed_tick.is_some()).count();
        let total_attack_steps = attacks.count();
        let successful_steps = attacks.successful().len();
        let detected_steps = attacks.detected().len();
        let total_detections = detections.count();
        let true_positive_detections = detections.true_positives().len();
        let detection_rate = detections.detection_rate();
        let success_rate = if total_attack_steps == 0 {
            0.0
        } else {
            successful_steps as f64 / total_attack_steps as f64
        };

        Self {
            total_scenarios,
            completed_scenarios,
            total_attack_steps,
            successful_steps,
            detected_steps,
            total_detections,
            true_positive_detections,
            detection_rate,
            success_rate,
        }
    }

    pub fn evasion_rate(&self) -> f64 {
        if self.total_attack_steps == 0 { return 0.0; }
        let undetected = self.total_attack_steps.saturating_sub(self.detected_steps);
        undetected as f64 / self.total_attack_steps as f64
    }
}
