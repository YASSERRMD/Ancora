use crate::checkpoint::EnterpriseCheckpoint;
use crate::incident::IncidentLog;
use crate::license::EnterpriseLicense;
use crate::posture::SecurityPosture;

pub struct EnterpriseStats {
    pub total_licenses: usize,
    pub active_licenses: usize,
    pub total_incidents: usize,
    pub open_incidents: usize,
    pub critical_incidents: usize,
    pub resolved_incidents: usize,
    pub total_checks: usize,
    pub passing_checks: usize,
    pub failing_checks: usize,
    pub overall_posture_score: u8,
    pub check_pass_rate: f64,
}

impl EnterpriseStats {
    pub fn compute(
        licenses: &[&EnterpriseLicense],
        incidents: &IncidentLog,
        checkpoint: &EnterpriseCheckpoint,
        posture: &SecurityPosture,
        current_tick: u64,
    ) -> Self {
        let total_licenses = licenses.len();
        let active_licenses = licenses.iter().filter(|l| l.is_valid(current_tick)).count();
        let total_incidents = incidents.count();
        let open_incidents = incidents.open().len();
        let critical_incidents = incidents.critical().len();
        let resolved_incidents = incidents.resolved().len();
        let total_checks = checkpoint.count();
        let passing_checks = checkpoint.passing().len();
        let failing_checks = checkpoint.failing().len();
        let check_pass_rate = checkpoint.pass_rate();
        let overall_posture_score = posture.overall_score();

        Self {
            total_licenses,
            active_licenses,
            total_incidents,
            open_incidents,
            critical_incidents,
            resolved_incidents,
            total_checks,
            passing_checks,
            failing_checks,
            overall_posture_score,
            check_pass_rate,
        }
    }

    pub fn health_score(&self) -> f64 {
        let posture_component = self.overall_posture_score as f64 / 100.0;
        let check_component = self.check_pass_rate;
        let incident_penalty = if self.critical_incidents > 0 {
            0.3
        } else if self.open_incidents > 0 {
            0.1
        } else {
            0.0
        };
        ((posture_component + check_component) / 2.0 - incident_penalty).max(0.0)
    }
}
