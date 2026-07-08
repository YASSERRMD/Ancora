use crate::checkpoint::{CheckStatus, EnterpriseCheckpoint, HealthCheck};
use crate::incident::{EnterpriseIncident, IncidentLog, IncidentSeverity};
use crate::license::{EnterpriseLicense, LicenseTier};
use crate::posture::{DomainScore, SecurityPosture};
use crate::stats::EnterpriseStats;

fn chk(id: &str, status: CheckStatus) -> HealthCheck {
    HealthCheck::new(id, "N", "domain", status, "msg", 1)
}

#[test]
fn empty_stats() {
    let licenses: Vec<&EnterpriseLicense> = vec![];
    let incidents = IncidentLog::new();
    let mut cp = EnterpriseCheckpoint::new(1);
    cp.add(chk("c1", CheckStatus::Pass));
    let posture = SecurityPosture::new("t1", 1);
    let stats = EnterpriseStats::compute(&licenses, &incidents, &cp, &posture, 1);
    assert_eq!(stats.total_licenses, 0);
    assert_eq!(stats.total_incidents, 0);
    assert_eq!(stats.overall_posture_score, 0);
}

#[test]
fn populated_stats() {
    let lic1 = EnterpriseLicense::new("l1", "t1", LicenseTier::Enterprise, 100, 1, 1);
    let lic2 = EnterpriseLicense::new("l2", "t1", LicenseTier::Community, 10, 1, 1).with_expiry(5);
    let licenses = vec![&lic1, &lic2];

    let mut incidents = IncidentLog::new();
    incidents.record(EnterpriseIncident::new(
        "i1",
        "t1",
        "N",
        IncidentSeverity::Critical,
        "d",
        1,
    ));
    incidents.record(EnterpriseIncident::new(
        "i2",
        "t1",
        "N",
        IncidentSeverity::Low,
        "d",
        1,
    ));

    let mut cp = EnterpriseCheckpoint::new(1);
    cp.add(chk("c1", CheckStatus::Pass));
    cp.add(chk("c2", CheckStatus::Fail));

    let mut posture = SecurityPosture::new("t1", 1);
    posture.add_domain(DomainScore::new("hsm", 80, 0, 0));

    let stats = EnterpriseStats::compute(&licenses, &incidents, &cp, &posture, 10);
    assert_eq!(stats.total_licenses, 2);
    assert_eq!(stats.active_licenses, 1); // lic2 expired at 5
    assert_eq!(stats.total_incidents, 2);
    assert_eq!(stats.open_incidents, 2);
    assert_eq!(stats.critical_incidents, 1);
    assert_eq!(stats.total_checks, 2);
    assert_eq!(stats.passing_checks, 1);
    assert_eq!(stats.failing_checks, 1);
    assert_eq!(stats.overall_posture_score, 80);
    assert!((stats.check_pass_rate - 0.5).abs() < 1e-9);
    // health penalized by critical incident
    assert!(stats.health_score() < 0.5);
}
