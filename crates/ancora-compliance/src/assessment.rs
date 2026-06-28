use crate::audit::{AssessmentRecord, ComplianceAuditLog};
use crate::control::ControlStatus;
use crate::framework::{ControlId, Framework};
use crate::registry::ControlRegistry;

pub struct AssessmentResult {
    pub control_id: ControlId,
    pub new_status: ControlStatus,
    pub tick: u64,
}

pub struct AutoAssessor;

impl AutoAssessor {
    pub fn bulk_mark_compliant(
        registry: &mut ControlRegistry,
        audit: &mut ComplianceAuditLog,
        control_ids: &[&str],
        framework: &Framework,
        tenant_id: &str,
        assessor: &str,
        tick: u64,
    ) -> Vec<AssessmentResult> {
        let mut results = Vec::new();
        for id_str in control_ids {
            let id = ControlId::new(*id_str);
            if let Some(ctrl) = registry.get_mut(&id) {
                let old = ctrl.status.clone();
                ctrl.set_status(ControlStatus::Compliant, tick);
                audit.record(AssessmentRecord::new(
                    tick, tenant_id, id.clone(), framework.clone(), old, ControlStatus::Compliant, assessor,
                ));
                results.push(AssessmentResult { control_id: id, new_status: ControlStatus::Compliant, tick });
            }
        }
        results
    }

    pub fn load_preset(
        registry: &mut ControlRegistry,
        controls: Vec<crate::control::ComplianceControl>,
    ) {
        for c in controls { registry.register(c); }
    }
}
