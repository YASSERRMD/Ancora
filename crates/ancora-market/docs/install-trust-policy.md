# Install Trust Policy

Operators configure an `InstallPolicy` that controls which extensions may be
installed in their environment. The policy is evaluated every time an extension
install is requested.

## Policy Configuration

```rust
use ancora_market::policy::{InstallPolicy, PolicyMode};
use ancora_market::badge::BadgeKind;
use ancora_market::residency::Region;

let policy = InstallPolicy {
    mode: PolicyMode::Strict,
    min_trust_score: 70,
    required_badges: vec![BadgeKind::SecurityVerified],
    allowed_regions: vec![Region::EEA, Region::LocalOnly],
};
```

## Policy Modes

### Permissive

All extensions are installable regardless of trust score. Trust signals are
recorded in the audit log but do not block installs. Useful for development
environments.

### Warn

Extensions below `min_trust_score` generate a warning that is surfaced to the
operator, but the install proceeds. Required badges and residency constraints
are not enforced. Suitable for staging environments.

### Strict

Extensions are blocked if any of the following conditions apply:
- Trust score is below `min_trust_score`.
- Any badge listed in `required_badges` is absent.
- The extension declares storage or processing regions not in `allowed_regions`.
- The residency declaration is missing when `allowed_regions` is non-empty.

Suitable for production and regulated environments.

## Verdict Handling

The `evaluate_policy` function returns a `PolicyVerdict`:

- `Allow` - proceed with install.
- `Warn(reasons)` - proceed but log each reason string.
- `Block(reasons)` - abort install and surface each reason string to the user.

## Recommended Defaults

| Environment | Mode       | min_trust_score |
|-------------|------------|-----------------|
| Development | Permissive | 0               |
| Staging     | Warn       | 50              |
| Production  | Strict     | 70              |
| Regulated   | Strict     | 80              |
