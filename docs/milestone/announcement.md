# Advanced Capabilities Milestone: Ancora v0.7.0

We are pleased to announce the Advanced Capabilities milestone for the Ancora
Rust agent framework.

## What is included

**15 new crates** shipping offline, deterministic, and fully-tested advanced
agent capabilities:

- Planning, reflection, and routing quality metrics
- Episodic and semantic memory with configurable consolidation
- Tool synthesis with a logical sandbox and audit trail
- Skill JIT loading and crew management
- Long-horizon checkpoints with configurable cadence
- Multi-agent coordination (contract-net, deadlock detection)
- Layered guardrails: injection detection, PII, allow/deny, custom policies
- Structured reasoning chains with citations and fact-checking
- 7-metric behavior evaluation framework
- Adversarial red-team harness with 5 scenario categories
- Cross-language parity validation (Rust canonical, Go validated)
- Capability presets for 5 common use cases, including a government-compliant air-gapped preset
- Performance benchmarks with regression-gated thresholds

## Test counts

| Suite | Tests |
|---|---|
| ancora-advdet (determinism) | 71 |
| ancora-advpar (parity) | 94 |
| ancora-preset (presets) | 75 |
| ancora-advbench (benchmarks) | 45 |
| ancora-redteam (red-team) | ~30 |

## How to start

```bash
cargo test -p ancora-preset -p ancora-advdet -p ancora-advpar -p ancora-advbench
```

All suites must pass offline with no network access.

## What comes next

Phase 201 begins enterprise and government hardening: SSO/OIDC, RBAC, ABAC,
audit trails, tenant isolation, and compliance tooling.
