# Self-hosting a Registry

ancora-registry is designed to be self-hosted: you spin up a single-binary
process on your own infrastructure and it serves the registry API over a
local network.

## Quick start

```rust
use ancora_registry::service::{RegistryConfig, RegistryService};

let config = RegistryConfig {
    name: "my-org-registry".to_string(),
    ..Default::default()
};
let mut registry = RegistryService::new(config);
```

## Configuration options

| Field | Default | Description |
|---|---|---|
| `name` | `"default"` | Human-readable name for this registry instance. |
| `strict_signatures` | `false` | Reject entries that carry no valid signature. |
| `airgap_mode` | `Online` | Network posture: `Online`, `AirGapped`, or `Private`. |
| `access_policy` | `Open` | Publisher allow-list or deny-all policy. |

## Access control

Restrict publishing to a known set of identities by choosing an
`AllowList` policy:

```rust
use ancora_registry::access_control::AccessPolicy;

let policy = AccessPolicy::allow_list(["ci-bot", "release-eng"]);
```

## Storage

The current implementation uses an in-memory store. Persistence can be
layered on top by serializing the registry state to disk and reloading it
on restart.
