use ancora_tenant::{
    AdmissionController, IsolationChecker, QuotaUpdate, ResourceQuota, TenantBuilder, TenantEvent,
    TenantEventKind, TenantEventLog, TenantRegistry,
};

fn main() {
    let mut registry = TenantRegistry::new();
    let mut event_log = TenantEventLog::new();
    let mut tick = 0u64;

    let (t1, q1) = TenantBuilder::new("acme", "Acme Corp", {
        tick += 1;
        tick
    })
    .metadata("plan", "enterprise")
    .metadata("region", "us-east-1")
    .quota(ResourceQuota::standard())
    .build();
    registry.register(t1, q1).unwrap();
    event_log.record(TenantEvent::new(tick, "acme", TenantEventKind::Registered));

    let (t2, q2) = TenantBuilder::new("beta", "Beta Ltd", {
        tick += 1;
        tick
    })
    .metadata("plan", "starter")
    .quota(ResourceQuota::restricted())
    .build();
    registry.register(t2, q2).unwrap();
    event_log.record(TenantEvent::new(tick, "beta", TenantEventKind::Registered));

    {
        let usage = registry.usage_mut("acme").unwrap();
        usage.agents = 3;
        usage.tasks = 15;
    }

    let quota = registry.quota("acme").unwrap();
    let usage = registry.usage("acme").unwrap();
    let d1 = AdmissionController::check_agents(quota, usage, 2);
    println!("Admit 2 more agents for acme: {:?}", d1);

    let d2 = AdmissionController::check_agents(quota, usage, 20);
    println!("Admit 20 more agents for acme: {:?}", d2);

    let isolation = IsolationChecker::require_same_tenant(&registry, "acme", "beta");
    println!("Cross-tenant check: {:?}", isolation);

    let same = IsolationChecker::require_same_tenant(&registry, "acme", "acme");
    println!("Same-tenant check: {:?}", same);

    let ns = registry.namespace_mut("acme").unwrap();
    ns.set("db_url", "postgres://internal/acme");
    ns.set("api_key", "sk-acme-abc123");
    println!("Acme namespace has {} keys", ns.count());
    println!("Scoped key: {}", ns.scoped_key("db_url"));

    registry.get_mut("beta").unwrap().suspend();
    event_log.record(TenantEvent::new(tick, "beta", TenantEventKind::Suspended));

    println!("\nActive tenants: {}", registry.active_tenants().len());
    println!("Suspended tenants: {}", registry.suspended_tenants().len());

    let mut acme_quota = registry.quota("acme").unwrap().clone();
    QuotaUpdate::new()
        .agents(20)
        .tasks(200)
        .apply(&mut acme_quota);
    println!("\nUpdated acme max_agents to {}", acme_quota.max_agents);

    println!("\nTotal events: {}", event_log.count());
    for e in event_log.all() {
        println!("  [tick={}] {} -> {:?}", e.tick, e.tenant_id, e.kind);
    }
}
