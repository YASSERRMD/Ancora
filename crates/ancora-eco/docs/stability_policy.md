# Extension API Stability Policy

The ancora-eco crate defines four stability levels for extension API points:

- **Unstable**: No guarantees. May change or be removed without notice.
- **Experimental**: Breaking changes require one deprecation cycle notice.
- **Stable**: Breaking changes require at least two deprecation cycles and an accepted RFC.
- **Frozen**: The API is locked and will not change. Any proposed change requires an RFC and unanimous core maintainer approval.

## Deprecation Cycles

A deprecation cycle corresponds to a minor release. When an API is marked
deprecated, the deprecation marker records the `since` version and the
`removed_in` version. Extensions using the API receive compile-time or
runtime warnings during the deprecation window.

## Enforcement

CI checks via `ancora-eco::breaking_detector::ci_check_stability` enforce
that no breaking change is merged unless the stability policy permits it.
