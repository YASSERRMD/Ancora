use ancora_netpol::{
    presets, ConnectionRequest, EvaluationRecord, NetpolAuditLog, NetpolStats, NetworkPolicy,
    PolicyEvaluator, PolicyValidator, RuleBuilder,
};

fn main() {
    let tenant = "acme";

    let mut policy = NetworkPolicy::deny_by_default(tenant);

    let rules = vec![
        RuleBuilder::new("allow-https")
            .host("*")
            .port(443)
            .tcp()
            .allow()
            .priority(100)
            .description("allow all outbound HTTPS")
            .build(),
        RuleBuilder::new("allow-internal-http")
            .host("*.internal.acme.corp")
            .port(80)
            .tcp()
            .allow()
            .priority(110)
            .description("allow internal HTTP")
            .build(),
    ];
    policy.bulk_add_rules(rules);

    presets::block_known_bad(&mut policy, "malware.example.com");

    let issues = PolicyValidator::validate(&policy);
    if issues.is_empty() {
        println!("Policy is valid: {} rules", policy.rule_count());
    } else {
        for issue in &issues {
            println!("Issue: {}", issue.description);
        }
    }

    let requests = vec![
        ConnectionRequest::tcp(tenant, "agent-01", "api.stripe.com", 443),
        ConnectionRequest::tcp(tenant, "agent-01", "db.internal.acme.corp", 80),
        ConnectionRequest::tcp(tenant, "agent-02", "malware.example.com", 443),
        ConnectionRequest::tcp(tenant, "agent-03", "external.com", 80),
    ];

    let mut audit = NetpolAuditLog::new();
    for (tick, req) in requests.iter().enumerate() {
        let decision = PolicyEvaluator::evaluate(&policy, req);
        let allowed = PolicyEvaluator::is_allowed(&policy, req);
        println!(
            "  [{:?}] {}:{} -> {}",
            decision,
            req.destination_host,
            req.destination_port,
            if allowed { "ALLOWED" } else { "DENIED" },
        );
        audit.record(EvaluationRecord::from(tick as u64, req, &decision));
    }

    let stats = NetpolStats::from_log(&audit, tenant);
    println!(
        "\nStats for '{}': {}/{} allowed ({:.0}% deny rate)",
        tenant,
        stats.allowed,
        stats.total,
        stats.deny_rate() * 100.0,
    );

    println!("\nPreset: HTTPS-only policy");
    let https_policy = presets::allow_https_only("demo-tenant");
    let test_req = ConnectionRequest::tcp("demo-tenant", "agent", "api.example.com", 443);
    println!(
        "  443 allowed: {}",
        PolicyEvaluator::is_allowed(&https_policy, &test_req)
    );
    let http_req = ConnectionRequest::tcp("demo-tenant", "agent", "api.example.com", 80);
    println!(
        "   80 denied: {}",
        !PolicyEvaluator::is_allowed(&https_policy, &http_req)
    );
}
