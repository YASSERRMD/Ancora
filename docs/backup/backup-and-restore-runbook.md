# Backup and Restore Runbook

## Backup types

| Type | Description | Use when |
|------|-------------|----------|
| Full snapshot | All journal entries | Weekly or before major changes |
| Incremental | Entries since last backup | Hourly or nightly |
| Point-in-time | Entries up to a specific seq | Investigating data corruption |

## Create a full snapshot

```rust
use ancora_backup::{BackupEngine, Journal};

let engine = BackupEngine::plaintext(); // or BackupEngine::new(encryption_key)
let archive = engine.snapshot(&journal, memory_kv, config_kv, now_secs)?;
// Store `archive` to disk or object storage
```

## Create an incremental backup

```rust
let last_seq = 1000; // seq of the last full backup
let archive = engine.incremental(&journal, last_seq, now_secs)?;
```

## Restore from snapshot

```rust
let mut journal = Journal::new();
engine.restore_snapshot(&archive, &mut journal)?;
```

## Restore incremental on top of snapshot

```rust
engine.restore_snapshot(&base_archive, &mut journal)?;
engine.restore_incremental(&incr_archive, &mut journal)?;
```

## Point-in-time restore

```rust
// Restore only up to seq 500
engine.restore_pit(&archive, &mut journal, 500)?;
```

## Schedule automated backups

```rust
use ancora_backup::BackupSchedule;

let mut sched = BackupSchedule::new(3600); // every hour
if sched.is_due(now_secs) {
    let archive = engine.snapshot(&journal, vec![], vec![], now_secs)?;
    // persist archive
    sched.record_run(now_secs);
}
```

## Integrity check

Every restore path verifies the SHA-256 checksum of the archive payload before
deserializing. A `BackupError::ChecksumMismatch` means the archive was corrupted
or tampered with; do not restore from it.
