# Offline Sync Model

ancora-sync implements a local-first architecture where edge devices continue
operating without network connectivity and synchronise with a central hub when
connectivity is restored.

## Core Concepts

### Journal

Every write on the device is appended to a local `Journal` as a `JournalEntry`
with a monotonically increasing sequence number. Entries start with a
`SyncMarker::Pending` status.

### Sync Markers

| Marker | Meaning |
|--------|---------|
| `Pending` | Not yet uploaded to the hub |
| `InFlight { attempt }` | Currently being uploaded (retry count tracked) |
| `Synced { hub_seq }` | Acknowledged by the hub; hub assigned sequence number stored |

### Change Log

While offline the `ChangeLog` records every mutation (Put / Update / Delete)
in chronological order. On reconnect the sync engine replays the change log
to compute the final state before uploading entries to the hub.

### Protocol

Sync uses a request / response model:

```
Device                           Hub
  |---[SyncRequest(entries)]---->|
  |<--[SyncResponse(acked)]------|
```

The protocol is transport-agnostic: the `SyncRequest` / `SyncResponse` structs
are serialised (e.g. with `serde_json`) and can be carried over any transport.

### Idempotency

Uploads are idempotent. The hub deduplicates entries by `(device_id, seq)`.
Uploading the same entry multiple times results in exactly one stored copy.

### Determinism

Given the same sequence of journal entries, the hub will reach the same final
state regardless of the number of upload attempts. Change-log replay is also
deterministic: applying the same change records to the same starting store
always produces the same result.
