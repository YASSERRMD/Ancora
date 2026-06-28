# Support and Versioning Policy

## Version scheme

Ancora follows [Semantic Versioning](https://semver.org/):

- **MAJOR**: breaking API or journal format changes.
- **MINOR**: new features, new providers, new backends, additive SDK additions.
- **PATCH**: bug fixes, security patches, documentation corrections.

## Support lifecycle

| Release type | Support window | Security patches |
|-------------|----------------|-----------------|
| Latest minor | Until next minor | Yes |
| Previous minor | 6 months after successor | Critical only |
| Older releases | End of life | No |

## Current supported releases

As of 2026-06-28:

| Version | Status | EOL |
|---------|--------|-----|
| 0.6.x | Active | TBD |
| 0.5.x | Security only | 2026-12-28 |
| <= 0.4.x | End of life | - |

## Stability guarantees

- The journal format version ("1") will not change in a patch or minor release.
- The A2A envelope schema (`a2a/1.0`) will not change in a minor release.
- The cost formula coefficients may change in a minor release (always documented in changelog).
- The OTel span field set is additive-only in minor releases.

## Deprecation policy

Features are deprecated for at least one minor release before removal.
Deprecated items are marked with `#[deprecated]` in Rust and equivalent annotations in other SDKs.
The changelog lists all deprecations under "Deprecated".
