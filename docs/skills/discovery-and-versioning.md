# Discovery and Versioning

## Tag-Based Discovery

Skills declare capability tags and are found by tag at runtime:

```rust
let retrieval_skills = registry.by_tag("retrieval");
```

## Versioned Resolution

Calls to `find` return the latest version. Use `find_version` to pin a version:

```rust
let v1 = registry.find_version("search", 1)?;
let latest = registry.find("search")?; // returns highest version
```

## Version Guidelines

- Increment version on breaking schema changes.
- Old versions remain in the registry for deterministic replay.
- Crew resolution always calls `lookup` which returns the latest version.
