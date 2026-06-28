use crate::framework::{ControlId, Framework};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlStatus {
    NotAssessed,
    Compliant,
    NonCompliant,
    PartiallyCompliant,
    NotApplicable,
}

impl std::fmt::Display for ControlStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlStatus::NotAssessed => write!(f, "NOT_ASSESSED"),
            ControlStatus::Compliant => write!(f, "COMPLIANT"),
            ControlStatus::NonCompliant => write!(f, "NON_COMPLIANT"),
            ControlStatus::PartiallyCompliant => write!(f, "PARTIALLY_COMPLIANT"),
            ControlStatus::NotApplicable => write!(f, "NOT_APPLICABLE"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComplianceControl {
    pub id: ControlId,
    pub framework: Framework,
    pub title: String,
    pub description: String,
    pub status: ControlStatus,
    pub evidence_ids: Vec<String>,
    pub assessed_tick: Option<u64>,
}

impl ComplianceControl {
    pub fn new(
        id: impl Into<String>,
        framework: Framework,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: ControlId::new(id),
            framework,
            title: title.into(),
            description: description.into(),
            status: ControlStatus::NotAssessed,
            evidence_ids: Vec::new(),
            assessed_tick: None,
        }
    }

    pub fn set_status(&mut self, status: ControlStatus, tick: u64) {
        self.status = status;
        self.assessed_tick = Some(tick);
    }

    pub fn attach_evidence(&mut self, evidence_id: impl Into<String>) {
        self.evidence_ids.push(evidence_id.into());
    }

    pub fn is_compliant(&self) -> bool {
        self.status == ControlStatus::Compliant
    }

    pub fn evidence_count(&self) -> usize {
        self.evidence_ids.len()
    }
}
