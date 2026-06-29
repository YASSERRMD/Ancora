# Residency-Aware Sync

Data residency laws (GDPR, data sovereignty requirements) may prohibit certain
data from leaving a geographic region. ancora-sync enforces residency rules
before any data leaves the device.

## Residency Zones

| Zone | Meaning |
|------|---------|
| `Global` | Data may be stored on any hub |
| `Region("EU")` | Data must remain within the named region |
| `Local` | Data must never leave the originating device |

## How It Works

Each `JournalEntry` is associated with a `ResidencyTag`. Before building a
`SyncRequest`, the `ResidencyFilter` checks every entry against the hub's zone:

1. `Global` entries are always allowed.
2. `Region(r)` entries are allowed only when the hub is in the same region `r`.
3. `Local` entries are **never** included in any sync request.

## Usage

```rust
use ancora_sync::model::ResidencyZone;
use ancora_sync::residency::{ResidencyFilter, ResidencyTag};

let tags = vec![
    ResidencyTag { seq: 1, zone: ResidencyZone::Region("EU".into()) },
    ResidencyTag { seq: 2, zone: ResidencyZone::Global },
];
let filter = ResidencyFilter::new(ResidencyZone::Region("US".into()), tags);
let request = filter.build_request("device-id", &journal_entries);
// request.entries will only contain the Global entry.
```

## Design Notes

* The `ResidencyFilter` operates on the device side, ensuring that restricted
  data never enters the network layer.
* Hub-side enforcement (verifying that incoming entries match the hub's zone
  policy) can be added as an additional guard layer.
