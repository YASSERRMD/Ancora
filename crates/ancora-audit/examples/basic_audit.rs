use ancora_audit::{
    summarize_by_tenant, to_csv, to_json, AuditEntryBuilder, AuditQuery, AuditStats,
    ImmutableAuditLog, Outcome, RetentionPolicy, Severity,
};

fn main() {
    let mut log = ImmutableAuditLog::new().with_max_size(1000);
    let mut tick = 0u64;

    let events = vec![
        (
            "tenant-a",
            "alice",
            "agent:execute",
            "agent-01",
            Outcome::Success,
            Severity::Info,
        ),
        (
            "tenant-a",
            "bob",
            "secret:read",
            "vault/db",
            Outcome::Blocked,
            Severity::Warning,
        ),
        (
            "tenant-b",
            "carol",
            "task:write",
            "task-99",
            Outcome::Success,
            Severity::Info,
        ),
        (
            "tenant-a",
            "alice",
            "user:delete",
            "user-55",
            Outcome::Failure,
            Severity::Error,
        ),
        (
            "tenant-b",
            "dave",
            "log:read",
            "syslog",
            Outcome::Success,
            Severity::Info,
        ),
        (
            "tenant-a",
            "eve",
            "agent:execute",
            "agent-02",
            Outcome::Success,
            Severity::Info,
        ),
    ];

    for (tenant, subject, op, resource, outcome, severity) in events {
        tick += 1;
        let entry = AuditEntryBuilder::new(tick, tenant, subject)
            .operation(op)
            .resource(resource)
            .outcome(outcome)
            .severity(severity)
            .detail("source", "example")
            .build();
        let id = log.append(entry);
        println!("appended id={}", id);
    }

    println!("\nAll checksums valid: {}", log.verify_all());

    let stats = AuditStats::from_entries(log.entries());
    println!("\nGlobal stats:");
    println!(
        "  total={} successes={} failures={} blocked={}",
        stats.total, stats.successes, stats.failures, stats.blocked
    );
    println!("  failure_rate={:.1}%", stats.failure_rate() * 100.0);
    println!("  critical={} errors={}", stats.critical, stats.errors);

    let failed = AuditQuery::new()
        .outcome(Outcome::Failure)
        .run(log.entries());
    println!("\nFailed operations:");
    for e in &failed {
        println!("  {}", e);
    }

    let entries_slice: Vec<&_> = log.entries().collect();
    let summaries = summarize_by_tenant(&entries_slice);
    println!("\nPer-tenant stats:");
    for s in &summaries {
        println!(
            "  tenant={} total={} failures={}",
            s.tenant_id, s.stats.total, s.stats.failures
        );
    }

    let retention = RetentionPolicy::new(3);
    let expired = retention.count_expired(&log, tick);
    println!("\nEntries older than 3 ticks: {}", expired);

    let all: Vec<&_> = log.entries().collect();
    let json = to_json(&all);
    println!("\nJSON export length: {} bytes", json.len());

    let csv = to_csv(&all);
    let lines = csv.lines().count();
    println!("CSV lines (including header): {}", lines);
}
