use crate::migration::Migration;
use crate::registry::MigrationRegistry;
use crate::runner::MigrationRunner;

fn build_runner(count: u32) -> MigrationRunner {
    let mut reg = MigrationRegistry::new();
    for v in 1..=count {
        reg.register(Migration::new(v, "ok", || Ok(()), || Ok(())));
    }
    MigrationRunner::new(reg)
}

#[test]
fn migrate_to_applies_all_pending() {
    let mut runner = build_runner(3);
    let applied = runner.migrate_to(3, 0).unwrap();
    assert_eq!(applied, 3);
    assert_eq!(runner.current_version(), 3);
}

#[test]
fn idempotent_second_migrate_applies_zero() {
    let mut runner = build_runner(2);
    runner.migrate_to(2, 0).unwrap();
    let applied = runner.migrate_to(2, 1).unwrap();
    assert_eq!(applied, 0);
}

#[test]
fn rollback_removes_migrations_above_target() {
    let mut runner = build_runner(3);
    runner.migrate_to(3, 0).unwrap();
    let rolled = runner.rollback_to(1, 10).unwrap();
    assert_eq!(rolled, 2);
    assert_eq!(runner.current_version(), 1);
}

#[test]
fn failing_migration_returns_error() {
    let mut reg = MigrationRegistry::new();
    reg.register(Migration::new(1, "ok", || Ok(()), || Ok(())));
    reg.register(Migration::new(2, "fail", || Err("boom".into()), || Ok(())));
    let mut runner = MigrationRunner::new(reg);
    assert!(runner.migrate_to(2, 0).is_err());
    assert_eq!(runner.current_version(), 1);
}
