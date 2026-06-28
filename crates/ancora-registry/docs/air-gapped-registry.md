# Air-gapped Registry

An air-gapped registry operates with no outbound network access. All entries
must be pushed in directly - no upstream proxying or mirroring from the
internet is attempted.

## Enabling air-gap mode

```rust
use ancora_registry::airgap::AirgapMode;
use ancora_registry::service::{RegistryConfig, RegistryService};

let config = RegistryConfig {
    airgap_mode: AirgapMode::AirGapped,
    ..Default::default()
};
let registry = RegistryService::new(config);
```

## Populating the registry offline

Use the mirror module to import a snapshot from an internet-connected
staging registry:

```rust
use ancora_registry::mirror::{MirrorSnapshot, MirrorStore, sync_from_snapshot};

// On the internet-connected machine: export a snapshot.
let snapshot: MirrorSnapshot = staging_store.to_snapshot();
// Transfer the snapshot (e.g., via USB, secure file transfer).
// On the air-gapped machine: import it.
let mut local_store = MirrorStore::default();
sync_from_snapshot(&mut local_store, &snapshot);
```

## Runtime behaviour

When `AirgapMode::AirGapped` is set, the `AirgapGuard` will return
`AirgapError::NetworkDisabled` for any operation that would normally make
an outbound call. Local fetches and searches operate normally.

## Private mode

`AirgapMode::Private` is a softer variant: the registry accepts pushes
from inside your network perimeter but does not forward any request to an
external upstream. Suitable for closed corporate environments where
developers do have network access but the registry itself should not
initiate outbound connections.
