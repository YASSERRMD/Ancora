# Security Review Notes for Advanced Capabilities

## Threat model

Advanced capabilities add new attack surfaces.  This document summarizes
the security considerations reviewed before the Phase 200 milestone.

## Injection guardrails

`InjectionInputGuardrail` blocks four known patterns:

- `"ignore previous instructions"`
- `"system prompt:"`
- `"jailbreak"`
- `"disregard all"`

**Gap:** new jailbreak patterns not in the list bypass the guard.
**Mitigation:** use the red-team harness (`ancora-redteam`) to test new attack
vectors before deploying new input channels.

## Tool synthesis sandbox

`SandboxRunner::execute` provides logical isolation but no OS-level sandboxing
in the current release.  Synthesized tools run in the same process.

**Recommendation:** in production, run `SandboxRunner` in a child process with
reduced capabilities (seccomp on Linux, App Sandbox on macOS).

## Memory consolidation

`ConsolidationJob` promotes episodic entries to semantic memory based on
occurrence count.  Adversarial agents could flood episodic memory with crafted
entries to influence what is promoted.

**Mitigation:** apply the `ForgettingPolicy` with a low `min_salience` to
filter low-quality entries before promotion.

## Skills JIT loading

`JitLoader` is a stub in this release.  When real JIT loading is added, ensure:

1. Skill descriptors are authenticated (signed or hash-verified)
2. `SkillScope::Unrestricted` skills are only loaded from trusted sources
3. Skill `input_schema` is validated before execution

## Government preset

See `docs/preset/government-preset-compliance.md` for a full compliance checklist.

The government preset excludes `Routing`, `ToolSynthesis`, `Skills`,
`Coordination`, and `CostControl` to prevent inadvertent external calls.

## Red-team coverage

`ancora-redteam` ships with 14 scenarios across 5 categories:
injection, tool misuse, data exfiltration, privilege escalation, jailbreak.

All 5 regression scenarios are always-blocked (effectiveness = 1.0) for the
InjectionInputGuardrail.  Add new scenarios for any new attack vectors before
deployment.
