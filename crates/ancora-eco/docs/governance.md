# Governance Model

The Ancora extension ecosystem uses a lightweight governance model inspired
by open-source best practices.

## Roles

- **Core Maintainers**: Responsible for the extension API specification, CI
  policy enforcement, and final RFC decisions. Require consensus (2/3 majority)
  for breaking changes.
- **Contributors**: Can propose RFCs, submit extensions to the registry, and
  participate in governance decisions via public comment.
- **Observers**: Read-only access to governance records, RFC discussions, and
  the compatibility matrix.

## Decision Process

1. A contributor or maintainer opens an RFC (see `rfc.md`).
2. The RFC enters a Final Comment Period (minimum 14 days).
3. Core Maintainers vote. A majority accept or reject.
4. Accepted RFCs are implemented and tracked in the governance ledger.

## Governance Ledger

All decisions are recorded as `GovernanceDecision` entries, providing an
auditable history of ecosystem changes.
