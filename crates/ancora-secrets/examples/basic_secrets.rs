use ancora_secrets::{
    AccessKind, AccessRecord, ExpiryChecker, RotationPolicy, SecretAccessLog, SecretKind,
    SecretStore,
};

fn main() {
    let mut store = SecretStore::new();
    let mut access_log = SecretAccessLog::new();
    let mut tick = 0u64;

    tick += 1;
    store
        .create(
            "acme",
            "database/prod/password",
            SecretKind::DatabaseCredential,
            "initial-pass-abc",
            tick,
        )
        .unwrap();
    access_log.record(AccessRecord::new(
        tick,
        "acme",
        "database/prod/password",
        "provisioner",
        AccessKind::Write,
    ));
    println!("Created database/prod/password at tick={}", tick);

    tick += 1;
    store
        .create(
            "acme",
            "api/stripe/key",
            SecretKind::ApiKey,
            "sk_live_initial",
            tick,
        )
        .unwrap();
    access_log.record(AccessRecord::new(
        tick,
        "acme",
        "api/stripe/key",
        "provisioner",
        AccessKind::Write,
    ));
    {
        let secret = store.read_mut("acme", "api/stripe/key").unwrap();
        secret.ttl_ticks = Some(200);
    }
    println!("Created api/stripe/key with TTL=200 ticks");

    tick += 1;
    let secret = store.read("acme", "database/prod/password").unwrap();
    println!(
        "\nActive database password: {}",
        secret.active_value().unwrap_or("NONE")
    );
    access_log.record(AccessRecord::new(
        tick,
        "acme",
        "database/prod/password",
        "agent-01",
        AccessKind::Read,
    ));

    tick += 10;
    let policy = RotationPolicy::default_policy();
    let new_ver = policy
        .rotate(
            &mut store,
            "acme",
            "database/prod/password",
            "rotated-pass-xyz",
            tick,
        )
        .unwrap();
    access_log.record(AccessRecord::new(
        tick,
        "acme",
        "database/prod/password",
        "rotation-bot",
        AccessKind::Rotate,
    ));
    println!("Rotated database/prod/password to version {}", new_ver);

    let secret = store.read("acme", "database/prod/password").unwrap();
    println!(
        "New active value: {}",
        secret.active_value().unwrap_or("NONE")
    );
    println!("Versions retained: {}", secret.version_count());

    println!("\nAll secrets for acme:");
    for s in store.list_tenant("acme") {
        println!(
            "  {} (versions: {}, active: {})",
            s.path,
            s.version_count(),
            s.active_version
        );
    }

    tick += 250;
    let expired = ExpiryChecker::expired_paths(&store, "acme", tick);
    println!("\nExpired at tick={}: {:?}", tick, expired);
    let active = ExpiryChecker::active_paths(&store, "acme", tick);
    println!("Active at tick={}: {:?}", tick, active);

    println!("\nAccess log for database/prod/password:");
    for r in access_log.all_for_path("database/prod/password") {
        println!("  tick={} subject={} kind={:?}", r.tick, r.subject, r.kind);
    }
}
