use crate::control::{ComplianceControl, ControlStatus};
use crate::framework::Framework;
use crate::registry::ControlRegistry;

#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub framework: Framework,
    pub tenant_id: String,
    pub generated_tick: u64,
    pub total_controls: usize,
    pub compliant: usize,
    pub non_compliant: usize,
    pub partially_compliant: usize,
    pub not_assessed: usize,
    pub not_applicable: usize,
}

impl ComplianceReport {
    pub fn generate(registry: &ControlRegistry, framework: &Framework, tenant_id: &str, tick: u64) -> Self {
        let controls: Vec<&ComplianceControl> = registry.for_framework(framework);
        let total = controls.len();
        let compliant = controls.iter().filter(|c| c.status == ControlStatus::Compliant).count();
        let non_compliant = controls.iter().filter(|c| c.status == ControlStatus::NonCompliant).count();
        let partially_compliant = controls.iter().filter(|c| c.status == ControlStatus::PartiallyCompliant).count();
        let not_applicable = controls.iter().filter(|c| c.status == ControlStatus::NotApplicable).count();
        let not_assessed = total - compliant - non_compliant - partially_compliant - not_applicable;
        Self {
            framework: framework.clone(),
            tenant_id: tenant_id.to_string(),
            generated_tick: tick,
            total_controls: total,
            compliant,
            non_compliant,
            partially_compliant,
            not_assessed,
            not_applicable,
        }
    }

    pub fn compliance_rate(&self) -> f64 {
        let assessed = self.total_controls - self.not_assessed - self.not_applicable;
        if assessed == 0 { return 0.0; }
        self.compliant as f64 / assessed as f64
    }

    pub fn is_fully_compliant(&self) -> bool {
        self.non_compliant == 0 && self.partially_compliant == 0 && self.not_assessed == 0
    }
}
