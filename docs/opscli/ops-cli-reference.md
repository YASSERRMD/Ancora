# Ancora Operations CLI Reference

The `ancora-opscli` library provides the data layer for operator tooling. An actual CLI binary would wrap these with argument parsing.

## Run management

```rust
use ancora_opscli::{RunStore, RunEntry, RunStatus, OutputFormat, render};

// List runs
let runs: Vec<&RunEntry> = store.list();
println!("{}", render(&runs, &OutputFormat::Table));

// Inspect a run
let run = store.get("run-abc").unwrap();

// Cancel a run
store.cancel("run-abc");

// Resume a failed/cancelled run
store.resume("run-abc");
```

## Worker management

```rust
use ancora_opscli::{WorkerRegistry, WorkerStatus, WorkerState};

// Status
let workers = registry.list();

// Drain
registry.drain("worker-1");
// Wait for registry.is_drained("worker-1") before removing
```

## Tenant management

```rust
use ancora_opscli::TenantOps;

let mut ops = TenantOps::default();
ops.create("tenant-a");
ops.suspend("tenant-a");
let all = ops.list();
```

## Backup

```rust
use ancora_opscli::BackupOps;

let mut bk = BackupOps::default();
let backup = bk.create_backup("tenant-a", now_secs);
println!("Backup {} checksum: {}", backup.id, backup.checksum);
```

## Output formats

All commands support `--output json` (default) and `--output table`:

```rust
use ancora_opscli::{OutputFormat, render};

let out = render(&data, &OutputFormat::Json);
let table = render(&data, &OutputFormat::Table);
```
