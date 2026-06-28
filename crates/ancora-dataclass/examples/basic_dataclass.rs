use ancora_dataclass::{
    AccessKind, ClassificationAuditEntry, ClassificationAuditLog, ClassificationEnforcer,
    ClassificationPolicy, DataCategory, DataQuery, DataRecordBuilder, DataRegistry,
    DowngradePolicy, EnforcementDecision, RedactionConfig, SensitivityLevel,
    to_csv, to_json,
};

fn main() {
    let tenant = "acme-corp";

    let policy = ClassificationPolicy::strict(tenant);

    let records = vec![
        DataRecordBuilder::new("r1", tenant, "Employee SSN")
            .level(SensitivityLevel::Restricted)
            .category(DataCategory::Pii)
            .tick(1)
            .tag("gdpr")
            .tag("pii")
            .build(),
        DataRecordBuilder::new("r2", tenant, "Public Blog Post")
            .level(SensitivityLevel::Public)
            .category(DataCategory::Generic)
            .tick(2)
            .build(),
        DataRecordBuilder::new("r3", tenant, "Internal API Key")
            .level(SensitivityLevel::Confidential)
            .category(DataCategory::Credentials)
            .tick(3)
            .tag("secrets")
            .build(),
    ];

    let mut registry = DataRegistry::new();
    let mut audit = ClassificationAuditLog::new();

    for record in records {
        let decision = ClassificationEnforcer::check_write(&policy, &record);
        let allowed = ClassificationEnforcer::is_allowed(&decision);
        let entry = ClassificationAuditEntry::from(
            record.created_tick,
            tenant,
            "system",
            &record.id,
            record.level.clone(),
            AccessKind::Write,
            &decision,
        );
        audit.record(entry);
        if allowed {
            registry.insert(record).unwrap();
        } else {
            if let EnforcementDecision::Deny(ref reason) = decision {
                println!("Write denied: {reason}");
            }
        }
    }

    println!("Stored {} records", registry.count());
    println!("Denied writes: {}", audit.denied_for_tenant(tenant).len());

    let high_sensitivity = DataQuery::new()
        .min_level(SensitivityLevel::Confidential)
        .run(registry.all());
    println!("High-sensitivity records: {}", high_sensitivity.len());

    let redaction = RedactionConfig::new(SensitivityLevel::Restricted).with_mask("[REDACTED]");
    for r in registry.all() {
        let safe_name = redaction.apply(&r.name, &r.level);
        println!("  [{}] {} -> displayed as: {}", r.level, r.name, safe_name);
    }

    let downgrade_pol = DowngradePolicy::new(SensitivityLevel::Internal);
    if let Ok(r) = registry.get("r1") {
        let mut r_clone = r.clone();
        let result = downgrade_pol.apply(&mut r_clone, SensitivityLevel::Public);
        println!("Downgrade r1 to Public: {:?}", result);
    }

    let all_refs: Vec<_> = registry.all().collect();
    let csv = to_csv(&all_refs);
    println!("\nCSV export (first 200 chars):\n{}", &csv[..csv.len().min(200)]);

    let json = to_json(&all_refs);
    println!("\nJSON export: {}", &json[..json.len().min(200)]);
}
