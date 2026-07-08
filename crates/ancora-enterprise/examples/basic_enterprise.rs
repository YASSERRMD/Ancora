use ancora_enterprise::audit::{EnterpriseAction, EnterpriseAuditEntry, EnterpriseAuditLog};
use ancora_enterprise::incident::{EnterpriseIncident, IncidentLog, IncidentSeverity};
use ancora_enterprise::presets::{
    default_feature_registry, enterprise_license, healthy_posture, standard_checkpoint,
};
use ancora_enterprise::report::EnterpriseReport;
use ancora_enterprise::stats::EnterpriseStats;

fn main() {
    let license = enterprise_license("lic-001", "tenant-acme", 1);
    let features = default_feature_registry();
    let checkpoint = standard_checkpoint(100);
    let posture = healthy_posture("tenant-acme", 100);

    let mut incidents = IncidentLog::new();
    incidents.record(
        EnterpriseIncident::new(
            "inc-1",
            "tenant-acme",
            "Unusual API activity",
            IncidentSeverity::High,
            "pentest",
            80,
        )
        .with_assignee("security-team"),
    );

    let mut audit = EnterpriseAuditLog::new();
    audit.record(EnterpriseAuditEntry::new(
        1,
        "tenant-acme",
        EnterpriseAction::LicenseIssued,
        "admin",
        "Annual enterprise license",
    ));
    audit.record(EnterpriseAuditEntry::new(
        100,
        "tenant-acme",
        EnterpriseAction::CheckpointRun,
        "cron",
        "Daily health check",
    ));
    audit.record(EnterpriseAuditEntry::new(
        100,
        "tenant-acme",
        EnterpriseAction::PostureAssessed,
        "cron",
        "Security posture computed",
    ));

    let licenses = vec![&license];
    let stats = EnterpriseStats::compute(&licenses, &incidents, &checkpoint, &posture, 100);
    let report = EnterpriseReport::generate(
        "tenant-acme",
        &licenses,
        &incidents,
        &checkpoint,
        &posture,
        &audit,
        100,
    );

    println!("=== Enterprise Status Report ===");
    println!("Tenant:            {}", report.tenant_id);
    println!("Active licenses:   {}", report.active_licenses);
    println!("License tier:      {}", license.tier);
    println!("Capabilities:      {}", license.cap_count());
    println!();
    println!(
        "Security posture:  {}/100 ({})",
        report.posture_score,
        posture.posture_level()
    );
    println!(
        "Health checks:     {}/{} passing",
        checkpoint.passing().len(),
        checkpoint.count()
    );
    println!("Failing checks:    {}", report.failing_checks);
    println!();
    println!("Open incidents:    {}", report.open_incidents);
    println!("Critical:          {}", report.critical_incidents);
    println!("Audit entries:     {}", report.audit_entry_count);
    println!();
    println!("Health score:      {:.0}%", stats.health_score() * 100.0);
    println!("System healthy:    {}", report.is_healthy());
    println!();
    println!(
        "Features enabled:  {}/{}",
        features.enabled_count(),
        features.count()
    );
}
