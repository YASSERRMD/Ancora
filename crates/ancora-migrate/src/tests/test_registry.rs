use crate::migration::Migration;
use crate::registry::MigrationRegistry;

fn ok_migration(v: u32) -> Migration {
    Migration::new(v, "test", || Ok(()), || Ok(()))
}

#[test]
fn register_and_count() {
    let mut r = MigrationRegistry::new();
    r.register(ok_migration(1));
    r.register(ok_migration(2));
    assert_eq!(r.count(), 2);
}

#[test]
fn versions_asc_sorted() {
    let mut r = MigrationRegistry::new();
    r.register(ok_migration(3));
    r.register(ok_migration(1));
    r.register(ok_migration(2));
    assert_eq!(r.versions_asc(), vec![1, 2, 3]);
}

#[test]
fn validate_sequence_ok() {
    let mut r = MigrationRegistry::new();
    r.register(ok_migration(1));
    r.register(ok_migration(2));
    assert!(r.validate_sequence().is_ok());
}

#[test]
fn validate_sequence_gap_errors() {
    let mut r = MigrationRegistry::new();
    r.register(ok_migration(1));
    r.register(ok_migration(3));
    assert!(r.validate_sequence().is_err());
}
