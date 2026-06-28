use crate::audit::EnterpriseAuditLog;
use crate::checkpoint::{CheckStatus, EnterpriseCheckpoint, HealthCheck};
use crate::incident::{EnterpriseIncident, IncidentLog, IncidentSeverity};
use crate::license::{EnterpriseLicense, LicenseTier};
use crate::posture::{DomainScore, SecurityPosture};
use crate::report::EnterpriseReport;

fn chk(id: &str, status: CheckStatus) -> HealthCheck {
    HealthCheck::new(id, "N", "d", status, "m", 1)
}

#[test]
fn empty_report() {
    let licenses: Vec<&EnterpriseLicense> = vec![];
    let incidents = IncidentLog::new();
    let cp = EnterpriseCheckpoint::new(1);
    let posture = SecurityPosture::new("t1", 1);
    let audit = EnterpriseAuditLog::new();
    let r = EnterpriseReport::generate("t1", &licenses, &incidents, &cp, &posture, &audit, 99);
    assert_eq!(r.tenant_id, "t1");
    assert_eq!(r.active_licenses, 0);
    assert_eq!(r.total_incidents, 0);
    assert_eq!(r.posture_score, 0);
    assert_eq!(r.tick, 99);
    assert!(r.is_healthy());
}

#[test]
fn populated_report() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::Enterprise, 100, 1, 1);
    let licenses = vec![&lic];

    let mut incidents = IncidentLog::new();
    incidents.record(EnterpriseIncident::new("i1", "t1", "N", IncidentSeverity::High, "d", 1));

    let mut cp = EnterpriseCheckpoint::new(1);
    cp.add(chk("c1", CheckStatus::Pass));
    cp.add(chk("c2", CheckStatus::Fail));

    let mut posture = SecurityPosture::new("t1", 1);
    posture.add_domain(DomainScore::new("d", 75, 0, 0));

    let audit = EnterpriseAuditLog::new();
    let r = EnterpriseReport::generate("t1", &licenses, &incidents, &cp, &posture, &audit, 50);
    assert_eq!(r.active_licenses, 1);
    assert_eq!(r.total_incidents, 1);
    assert_eq!(r.open_incidents, 1);
    assert_eq!(r.check_count, 2);
    assert_eq!(r.failing_checks, 1);
    assert_eq!(r.posture_score, 75);
    assert!(!r.is_healthy());
}
