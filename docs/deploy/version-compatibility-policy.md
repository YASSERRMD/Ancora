# Worker Version Compatibility Policy

## Versioning scheme

Workers follow Semantic Versioning: `MAJOR.MINOR.PATCH`.

| Component | Rule |
|-----------|------|
| MAJOR | Breaking change to journal schema or wire protocol |
| MINOR | Backward-compatible new feature |
| PATCH | Bug fix only |

## Compatibility rules

- Workers with the **same major version** may share a journal store and
  run concurrently (mixed-version pool is safe).
- Workers with **different major versions** must not share a journal store.
  Use a blue-green switch to migrate between major versions.

## Upgrade path for major versions

1. Start green pool with new major version (empty journal store).
2. Drain all in-flight runs on the blue pool.
3. Perform a blue-green switch.
4. Copy the completed journal entries from blue to green if needed.
5. The old blue pool can be decommissioned after verification.

## Schema negotiation

Workers check compatibility before connecting:
```rust
assert_compatible(&my_version, &store_version)?;
```

If the check fails, the worker refuses to start and logs the version mismatch.
