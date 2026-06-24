# Wire-spec versioning and stability guarantees

## Semantic versioning

Ancora follows Semantic Versioning 2.0.0 for the wire spec and all
published artifacts (crates, language packages, the C header).

```
MAJOR.MINOR.PATCH
```

- **MAJOR**: incompatible changes to the C ABI, protobuf field numbers,
  or the journal event schema. Consumers must update their code.
- **MINOR**: backward-compatible additions: new proto fields (never
  reusing existing field numbers), new ABI functions, new event variants
  with the existing envelope. Existing bindings continue to work.
- **PATCH**: bug fixes, documentation updates, internal refactors that
  do not change any observable interface.

## Stability tiers

| Tier | Label | Guarantee |
|------|-------|-----------|
| Stable | (none) | Full SemVer guarantees above |
| Experimental | `#[experimental]` / `@experimental` | May change in any MINOR release; opt-in only |
| Internal | `#[doc(hidden)]` / `_` prefix | No stability guarantee; not part of the public API |

## Proto field number policy

- Field numbers 1-999 are reserved for core message types.
- Field numbers 1000-1999 are reserved for extension points.
- Field numbers 2000+ are available for experimental features.
- A field number is never reused once it has appeared in a released
  version, even after the field is removed. Removed fields are marked
  reserved in the .proto file.

## C ABI versioning

The C header exposes a `ancora_version()` function that returns the
wire-spec version string. Bindings should call this at startup and
refuse to operate if the MAJOR version does not match the version they
were built against.

New ABI functions are additive (MINOR bump). Removed or renamed
functions are a MAJOR bump and require a migration note in the changelog.

## Journal format versioning

The journal event envelope carries a `wire_version` field (MAJOR only).
The replay engine rejects journals with a higher MAJOR version than the
runtime supports. It can read journals with a lower MAJOR version if a
migration reader is registered (optional, not required for the initial
release).

## Changelog

All changes to the wire spec are documented in `CHANGELOG.md` under the
affected version, following the Keep a Changelog format. MAJOR and MINOR
changes include a migration guide.
