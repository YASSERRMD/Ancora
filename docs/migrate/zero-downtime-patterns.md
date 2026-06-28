# Zero-Downtime Migration Patterns

## Expand-Contract

The safest way to rename or remove a column from a live system.

| Step | Action | Impact |
|------|--------|--------|
| Expand | Add the new column (nullable) | None |
| Dual-write | Write to both old and new columns | Application change |
| Backfill | Copy old column values to new column | Background, batched |
| Read switch | Read from new column | Application change |
| Contract | Drop old column | Small rewrite |

Use `ZdtMigration` to track progress through this lifecycle.

## Adding a NOT NULL column

1. Add as nullable (`Expand`).
2. Set a default for existing rows (`Backfill`).
3. Add NOT NULL constraint after backfill (`Contract`).

Never add a NOT NULL column directly to a table with existing rows.

## Renaming a column

1. Add new column (`Expand`).
2. Write to both (`Dual-write` -- application layer).
3. Backfill new from old (`Backfill`).
4. Switch reads to new (`Read switch`).
5. Drop writes to old.
6. Drop old column (`Contract`).

## Index creation

Create indexes concurrently (non-blocking) during a maintenance window or as a background task. Never block the migration runner on an index build.

## Backfill batching

Process backfill in batches of 500-1000 rows and call `advance_backfill(batch_size)` each iteration to track progress. This prevents long-running transactions and keeps `progress_pct()` accurate.
