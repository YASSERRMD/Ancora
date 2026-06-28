# Governance and Versioning

Ancora follows Semantic Versioning (SemVer 2.0.0) for all public API surfaces.

## Stability tiers

| Tier         | Change policy                              |
|--------------|--------------------------------------------|
| Experimental | May change in any release without notice   |
| Unstable     | May change between minor versions          |
| Stable       | Breaking changes only in major bumps       |
| Deprecated   | Will be removed in the next major version  |

## Deprecation timeline

When a symbol is deprecated in version `X.Y.0`, it will be removed no earlier
than version `(X+1).0.0`. A minimum of two minor releases must pass between
deprecation and removal.

## Breaking changes

Breaking changes require a major version bump and must be documented in `CHANGELOG.md`
with a migration guide.
