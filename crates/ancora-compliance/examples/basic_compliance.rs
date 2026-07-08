use ancora_compliance::{
    controls_to_csv, presets, report_to_csv, AutoAssessor, ComplianceAuditLog, ComplianceReport,
    ControlRegistry, ControlStatus, EvidenceItem, EvidenceKind, EvidenceStore, Framework,
    GapAnalyzer,
};

fn main() {
    let tenant = "acme-corp";
    let mut registry = ControlRegistry::new();
    let mut audit = ComplianceAuditLog::new();
    let mut evidence = EvidenceStore::new();

    AutoAssessor::load_preset(&mut registry, presets::soc2_controls());
    println!(
        "Loaded {} SOC 2 controls",
        registry.for_framework(&Framework::Soc2).len()
    );

    evidence.insert(EvidenceItem::new(
        "ev-001",
        EvidenceKind::LogEntry,
        "Auth logs",
        "Authentication log export",
        10,
        tenant,
    ));
    evidence.insert(EvidenceItem::new(
        "ev-002",
        EvidenceKind::TestResult,
        "Access test",
        "RBAC integration test results",
        20,
        tenant,
    ));

    let compliant_ids = ["CC6.1", "CC6.2", "A1.1"];
    let results = AutoAssessor::bulk_mark_compliant(
        &mut registry,
        &mut audit,
        &compliant_ids,
        &Framework::Soc2,
        tenant,
        "alice",
        100,
    );
    println!("Marked {} controls compliant", results.len());

    if let Some(ctrl) = registry.get_mut(&ancora_compliance::ControlId::new("CC7.1")) {
        ctrl.set_status(ControlStatus::NonCompliant, 101);
        ctrl.attach_evidence("ev-001");
    }

    let report = ComplianceReport::generate(&registry, &Framework::Soc2, tenant, 200);
    println!("\nSOC 2 Report for '{}':", tenant);
    println!("  Total controls : {}", report.total_controls);
    println!("  Compliant      : {}", report.compliant);
    println!("  Non-compliant  : {}", report.non_compliant);
    println!("  Not assessed   : {}", report.not_assessed);
    println!(
        "  Compliance rate: {:.1}%",
        report.compliance_rate() * 100.0
    );
    println!("  Fully compliant: {}", report.is_fully_compliant());

    let gaps = GapAnalyzer::analyze(&registry, &Framework::Soc2);
    println!("\nGap analysis: {} gap(s)", gaps.len());
    for g in &gaps {
        println!("  [{}] {} - {:?}", g.control_id, g.title, g.status);
    }

    println!("\nCSV Report:\n{}", report_to_csv(&report));

    let all_controls: Vec<_> = registry.for_framework(&Framework::Soc2);
    println!("Controls CSV:\n{}", controls_to_csv(&all_controls));

    println!("Audit records: {}", audit.count());
    println!(
        "Evidence items for tenant: {}",
        evidence.for_tenant(tenant).len()
    );
}
