# Immutable Log Design

## Why append-only

The audit log must be trustworthy. If entries can be updated or deleted, an attacker who gains write access to the store can erase evidence of their activity. Append-only semantics bound the blast radius: a compromised subject can add false entries but cannot remove true ones.

## Checksum scheme

Each entry carries a `checksum` computed by mixing the `id`, `tick`, `tenant_id` bytes, and `operation` bytes through a sequence of wrapping multiplications using large prime-like constants. This is not a cryptographic hash; it is a low-overhead tamper indicator that detects accidental or deliberate in-memory mutation.

On `append`, the log overwrites the incoming entry's `id` and recomputes its checksum. This means the checksum is always authoritative over the stored values at the time of append.

`verify_all()` runs the checksum recomputation for every entry in O(n) time. Call it on a schedule or before serving sensitive queries.

## Monotonic tick vs wall-clock time

The log uses caller-supplied `tick` values (u64) rather than wall-clock timestamps. This avoids:

- Time-zone ambiguities
- Clock skew between services
- Non-monotonic jumps when the system clock is adjusted

The caller is responsible for providing a monotonically increasing tick. A common pattern is a global atomic counter incremented on each agent dispatch.

## Bounded capacity

`with_max_size(n)` caps the in-memory footprint. When the cap is reached the oldest entry is evicted by `pop_front` from the underlying `VecDeque`. The id sequence is never reset; evicted ids are gone and `get(id)` returns `None` for them. This property preserves id uniqueness and allows downstream archiving systems to detect gaps.

## Threat model

The immutable log resists these threats:

- In-memory mutation: checksum detects it.
- Entry deletion: append-only prevents it.
- Replay of old entries with forged ids: ids are assigned by the log, not the caller.

It does not resist:

- A privileged attacker who can restart the process with a fresh log.
- Side-channel observation of in-memory entries.
- Replacement of the entire log binary.

For higher assurance, emit log entries to an external append-only sink (e.g., write-once object storage) on each `append`.
