# Ecosystem Readiness Checklist

Use this checklist to assess whether your plugin is ready for a given milestone.

## Alpha release

- [ ] Crate compiles on stable Rust without warnings (ready-001)
- [ ] Test coverage covers happy path and at least two error cases (ready-003)

## Beta release (includes alpha)

- [ ] All public items documented with `///` comments (ready-002)
- [ ] CHANGELOG.md contains an entry for the current version (ready-006)

## Stable release (includes beta)

- [ ] No unsafe code, or unsafe blocks are documented and reviewed (ready-005)

## Marketplace listing (includes stable)

- [ ] Catalog entry passes validation (ready-004)

Run `ancora readiness check --milestone marketplace` to evaluate your plugin automatically.
