use crate::control::ControlStatus;
use crate::registry::ControlRegistry;
use crate::framework::Framework;

#[derive(Debug, Clone)]
pub struct ComplianceStats {
    pub framework: Framework,
    pub total: usize,
    pub compliant: usize,
    pub non_compliant: usize,
    pub not_assessed: usize,
}

impl ComplianceStats {
    pub fn from_registry(registry: &ControlRegistry, framework: &Framework) -> Self {
        let controls = registry.for_framework(framework);
        let total = controls.len();
        let compliant = controls.iter().filter(|c| c.status == ControlStatus::Compliant).count();
        let non_compliant = controls.iter().filter(|c| c.status == ControlStatus::NonCompliant).count();
        let not_assessed = controls.iter().filter(|c| c.status == ControlStatus::NotAssessed).count();
        Self { framework: framework.clone(), total, compliant, non_compliant, not_assessed }
    }

    pub fn compliance_rate(&self) -> f64 {
        if self.total == 0 { 0.0 } else { self.compliant as f64 / self.total as f64 }
    }

    pub fn gap_count(&self) -> usize { self.non_compliant + self.not_assessed }
}
