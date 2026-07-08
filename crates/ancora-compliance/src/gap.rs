use crate::control::{ComplianceControl, ControlStatus};
use crate::framework::Framework;
use crate::registry::ControlRegistry;

#[derive(Debug, Clone)]
pub struct GapItem {
    pub control_id: String,
    pub framework: Framework,
    pub title: String,
    pub status: ControlStatus,
    pub missing_evidence: usize,
}

pub struct GapAnalyzer;

impl GapAnalyzer {
    pub fn analyze(registry: &ControlRegistry, framework: &Framework) -> Vec<GapItem> {
        let mut gaps: Vec<GapItem> = registry
            .for_framework(framework)
            .into_iter()
            .filter(|c| {
                c.status != ControlStatus::Compliant && c.status != ControlStatus::NotApplicable
            })
            .map(|c: &ComplianceControl| GapItem {
                control_id: c.id.0.clone(),
                framework: c.framework.clone(),
                title: c.title.clone(),
                status: c.status.clone(),
                missing_evidence: if c.evidence_ids.is_empty() { 1 } else { 0 },
            })
            .collect();
        gaps.sort_by(|a, b| a.control_id.cmp(&b.control_id));
        gaps
    }

    pub fn critical_gaps(registry: &ControlRegistry, framework: &Framework) -> Vec<GapItem> {
        Self::analyze(registry, framework)
            .into_iter()
            .filter(|g| g.status == ControlStatus::NonCompliant && g.missing_evidence > 0)
            .collect()
    }
}
