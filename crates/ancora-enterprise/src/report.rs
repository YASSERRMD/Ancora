use crate::audit::EnterpriseAuditLog;
use crate::checkpoint::EnterpriseCheckpoint;
use crate::incident::IncidentLog;
use crate::license::EnterpriseLicense;
use crate::posture::SecurityPosture;

pub struct EnterpriseReport {
    pub tenant_id: String,
    pub active_licenses: usize,
    pub total_incidents: usize,
    pub open_incidents: usize,
    pub critical_incidents: usize,
    pub check_count: usize,
    pub failing_checks: usize,
    pub posture_score: u8,
    pub audit_entry_count: usize,
    pub tick: u64,
}

impl EnterpriseReport {
    pub fn generate(
        tenant_id: impl Into<String>,
        licenses: &[&EnterpriseLicense],
        incidents: &IncidentLog,
        checkpoint: &EnterpriseCheckpoint,
        posture: &SecurityPosture,
        audit: &EnterpriseAuditLog,
        tick: u64,
    ) -> Self {
        let tenant_str = tenant_id.into();
        let active_licenses = licenses
            .iter()
            .filter(|l| l.is_valid(tick) && l.tenant_id == tenant_str)
            .count();
        let tenant_incidents = incidents.for_tenant(&tenant_str);

        Self {
            tenant_id: tenant_str,
            active_licenses,
            total_incidents: tenant_incidents.len(),
            open_incidents: tenant_incidents.iter().filter(|i| i.is_open()).count(),
            critical_incidents: tenant_incidents.iter().filter(|i| i.is_critical()).count(),
            check_count: checkpoint.count(),
            failing_checks: checkpoint.failing().len(),
            posture_score: posture.overall_score(),
            audit_entry_count: audit.count(),
            tick,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.failing_checks == 0 && self.critical_incidents == 0
    }
}
