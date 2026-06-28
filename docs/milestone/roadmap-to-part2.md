# Roadmap to Enterprise and Government Hardening

## Phase 200 exit state

The advanced-capabilities section (Phases 161-200) is complete.  All crates are
offline, deterministic, and regression-gated.  The milestone tag is `v0.7.0`.

## Part 2: Enterprise and government hardening (Phases 201-220)

| Phase | Name | Goal |
|---|---|---|
| 201 | SSO | OIDC and SAML authentication with mock IdPs |
| 202 | RBAC | Role-based access control (admin, operator, developer, viewer) |
| 203 | ABAC | Attribute-based access control and policy engine |
| 204 | Audit | Immutable audit log for all operations |
| 205 | Tenant isolation | Per-tenant resource and data isolation |
| 206 | Secrets | Secrets management (vault integration, rotation) |
| 207 | Network policy | Egress controls and network policy enforcement |
| 208 | Data classification | Data sensitivity labels and enforcement |
| 209 | Compliance reporting | SOC 2, FedRAMP, ISO 27001 evidence collection |
| 210 | Key management | Encryption key lifecycle and HSM integration |
| 211 | Secure boot | Agent process integrity verification |
| 212 | Supply chain | Dependency signing and SBOM generation |
| 213 | Incident response | Automated runbooks for security incidents |
| 214 | Threat intel | IOC feeds and threat scoring |
| 215 | Zero trust | Zero-trust networking for agent-to-agent calls |
| 216 | HSM | Hardware security module integration |
| 217 | Air-gap ops | Operational procedures for classified environments |
| 218 | Pen-test | Automated penetration testing harness |
| 219 | Red team II | Extended adversarial scenarios for enterprise |
| 220 | Enterprise checkpoint | Consolidation, tag, and release |

## Priority areas for Part 2

1. **SSO (Phase 201)**: unblocks all enterprise deployments
2. **RBAC (Phase 202)**: required for multi-user environments
3. **Audit (Phase 204)**: compliance blocker for most regulated sectors
4. **Tenant isolation (Phase 205)**: SaaS deployment prerequisite

## Government preset evolution

The `government-compliant` preset introduced in Phase 198 will be hardened
through Phases 211 (secure boot) and 217 (air-gap operations procedures).
