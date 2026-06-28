# Deprecation Policy

## Overview

When an extension API point is deprecated, it enters a deprecation window
during which it continues to function but emits warnings. At the end of the
window it is removed.

## Requirements for Deprecation

- A `DeprecationMarker` must be attached to the API point with:
  - `since`: the version in which the deprecation was announced.
  - `removed_in`: the version in which the item will be removed.
  - `message`: a human-readable explanation and migration path.
- For **Stable** APIs, the deprecation window must span at least two minor
  release cycles.
- For **Experimental** APIs, one minor release cycle suffices.
- For **Unstable** APIs, items may be removed without a deprecation window.

## Warnings

Extensions using deprecated APIs will receive a `DeprecationWarning` at load
time. The warning includes the API name, the `since` version, and the
`removed_in` version.

## Enforcement

The CI pipeline checks all deprecated usages and fails if any deprecated API
has already passed its `removed_in` version.
