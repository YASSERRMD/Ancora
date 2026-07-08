use crate::control::ControlStatus;
use crate::framework::{ControlId, Framework};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AssessmentRecord {
    pub tick: u64,
    pub tenant_id: String,
    pub control_id: ControlId,
    pub framework: Framework,
    pub old_status: ControlStatus,
    pub new_status: ControlStatus,
    pub assessor: String,
}

impl AssessmentRecord {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        control_id: ControlId,
        framework: Framework,
        old_status: ControlStatus,
        new_status: ControlStatus,
        assessor: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            control_id,
            framework,
            old_status,
            new_status,
            assessor: assessor.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ComplianceAuditLog {
    records: VecDeque<AssessmentRecord>,
}

impl ComplianceAuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, entry: AssessmentRecord) {
        self.records.push_back(entry);
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn for_framework(&self, framework: &Framework) -> Vec<&AssessmentRecord> {
        self.records
            .iter()
            .filter(|r| &r.framework == framework)
            .collect()
    }

    pub fn for_control(&self, id: &ControlId) -> Vec<&AssessmentRecord> {
        self.records
            .iter()
            .filter(|r| &r.control_id == id)
            .collect()
    }

    pub fn for_tenant(&self, tenant_id: &str) -> Vec<&AssessmentRecord> {
        self.records
            .iter()
            .filter(|r| r.tenant_id == tenant_id)
            .collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &AssessmentRecord> {
        self.records.iter()
    }
}
