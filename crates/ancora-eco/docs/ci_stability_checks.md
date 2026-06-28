# Extension API Stability Checks in CI

## Overview

The `ancora-eco` CI pipeline runs extension API stability checks on every
pull request that modifies an extension hook, lifecycle callback, or version
negotiation protocol.

## Checks Performed

1. **Breaking Change Detection**: `ci_check_stability` compares the current
   API snapshot against the previous release snapshot. Any removed endpoints
   are flagged.

2. **Stability Policy Enforcement**: Breaking changes are only permitted if
   the stability policy for the affected API level allows them (e.g., Stable
   APIs require 2 deprecation cycles).

3. **Deprecation Window Validation**: All deprecated APIs are checked to
   ensure they have not passed their `removed_in` version without being
   actually removed.

4. **Compatibility Matrix**: The full extension registry is negotiated against
   the proposed new core version. A report is posted as a PR comment.

## Fail Conditions

CI fails if:
- A breaking change is detected in a Stable or Frozen API without sufficient
  deprecation cycles.
- A deprecated API still exists past its `removed_in` version.
- The build or tests fail.

## Running Locally

```bash
cargo build -p ancora-eco
cargo test -p ancora-eco
```
