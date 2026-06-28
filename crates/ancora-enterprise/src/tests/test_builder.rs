use crate::builder::{DomainScoreBuilder, HealthCheckBuilder, IncidentBuilder, LicenseBuilder};
use crate::checkpoint::CheckStatus;
use crate::incident::IncidentSeverity;
use crate::license::{EnterpriseCap, LicenseTier};

#[test]
fn license_builder() {
    let lic = LicenseBuilder::new("l1", "t1", LicenseTier::Enterprise, 10)
        .max_users(500)
        .max_tenants(5)
        .cap(EnterpriseCap::Hsm)
        .cap(EnterpriseCap::AirGap)
        .expires(1000)
        .build();
    assert_eq!(lic.max_users, 500);
    assert_eq!(lic.max_tenants, 5);
    assert_eq!(lic.cap_count(), 2);
    assert!(lic.has_cap(&EnterpriseCap::Hsm));
    assert_eq!(lic.expires_tick, Some(1000));
}

#[test]
fn incident_builder() {
    let inc = IncidentBuilder::new("i1", "t1", "Alert", IncidentSeverity::High, "pentest", 5)
        .assignee("alice")
        .build();
    assert_eq!(inc.id, "i1");
    assert_eq!(inc.assignee.as_deref(), Some("alice"));
    assert!(inc.is_critical() == false);
}

#[test]
fn health_check_builder() {
    let chk = HealthCheckBuilder::new("c1", "HSM check", "hsm", CheckStatus::Warn, 3)
        .message("Slot degraded")
        .build();
    assert_eq!(chk.id, "c1");
    assert_eq!(chk.message, "Slot degraded");
    assert!(chk.is_warning());
}

#[test]
fn domain_score_builder() {
    let ds = DomainScoreBuilder::new("pentest", 72)
        .findings(10)
        .critical(2)
        .build();
    assert_eq!(ds.domain, "pentest");
    assert_eq!(ds.score, 72);
    assert_eq!(ds.findings, 10);
    assert_eq!(ds.critical_findings, 2);
}
