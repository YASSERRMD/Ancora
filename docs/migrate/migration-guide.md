# Ancora Migration Guide

The `ancora-migrate` crate provides versioned schema migration with zero-downtime patterns.

## Defining migrations

```rust
use ancora_migrate::{Migration, MigrationRegistry, MigrationRunner};

let mut reg = MigrationRegistry::new();
reg.register(Migration::new(1, "create runs table",
    || { /* apply */ Ok(()) },
    || { /* rollback */ Ok(()) },
));
reg.register(Migration::new(2, "add tenant_id column",
    || Ok(()),
    || Ok(()),
));
reg.validate_sequence().expect("no gaps in version sequence");
```

## Running migrations

```rust
let mut runner = MigrationRunner::new(reg);
let applied = runner.migrate_to(2, now_secs)?;
println!("Applied {applied} migrations. Current version: {}", runner.current_version());
```

## Rolling back

```rust
let rolled = runner.rollback_to(1, now_secs)?;
println!("Rolled back {rolled} migrations.");
```

## Maintenance window

```rust
use ancora_migrate::MaintenanceWindow;

let mut mw = MaintenanceWindow::new();
mw.enter(now, "schema upgrade v2 -> v3");
// ... run migrations ...
mw.exit();
```

Gate run dispatch on `!mw.is_active()`.

## Zero-downtime expand-contract

```rust
use ancora_migrate::{ZdtMigration, ZdtPhase};

let mut m = ZdtMigration::new("add-model-column", total_rows);
m.start_expand();     // add nullable column (no lock)
m.start_backfill();   // async backfill
while !m.backfill_complete() {
    m.advance_backfill(batch_size);
}
m.start_contract();   // drop old column
m.finish();
```

## Migration lock

```rust
use ancora_migrate::MigrationLock;

let mut lock = MigrationLock::new(120); // 120s TTL
if lock.acquire("node-1", now) {
    runner.migrate_to(target, now)?;
    lock.release("node-1");
} else {
    println!("Migration already in progress by {}", lock.holder().unwrap_or("unknown"));
}
```
