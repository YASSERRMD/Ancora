# Publishing Workflow

This document describes the end-to-end lifecycle of publishing an entry to
an ancora-registry instance.

## 1. Prepare the entry

Assemble a `PublishEntry` with the entry name, semantic version, raw payload
bytes, and publisher identity:

```rust
use ancora_registry::publish::PublishEntry;
use ancora_registry::versioning::Version;

let entry = PublishEntry::new(
    "my-tool",
    Version::new(1, 2, 0),
    std::fs::read("my-tool.tar.gz").unwrap(),
    "release-bot",
);
```

## 2. Sign the entry (recommended)

When the registry is running in strict-signature mode you must attach a
valid detached signature. Generate the expected signature token using your
registered key:

```rust
use ancora_registry::signature::{SignatureStore, TrustedKey};

let key = TrustedKey::new("release-key", key_material);
let sig = SignatureStore::expected_sig(&key, "my-tool", &version);
let entry = entry.with_signature(sig);
```

## 3. Publish

Call `RegistryService::publish`. The registry performs access-control and
signature checks before persisting the entry:

```rust
registry.publish(entry).expect("publish failed");
```

## 4. Verify via fetch

After publishing, confirm the entry is retrievable:

```rust
use ancora_registry::fetch::FetchResult;

let result = registry.fetch("my-tool", &Version::new(1, 2, 0));
assert!(result.is_found());
```

## 5. Use the CLI

The registry ships a minimal command-line interface for scripted workflows:

```
publish my-tool 1.2.0 release-bot
fetch   my-tool 1.2.0
search  my-tool
versions my-tool
```
