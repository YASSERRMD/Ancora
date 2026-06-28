# Government Preset Readiness

## Status: Ready for evaluation

The `government-compliant` preset and all its dependencies are offline,
deterministic, and locked.  This document certifies readiness for evaluation
in government and classified environments.

## Readiness checklist

- [x] Air-gap enforced: `AirGapPolicy::Required`
- [x] Data residency zone: configurable via `government_compliant(zone)`
- [x] Capability set locked: `locked: true`
- [x] Routing excluded: no remote model dispatch
- [x] ToolSynthesis excluded: no external registry calls
- [x] Skills excluded: no remote skill loading
- [x] All 75 ancora-preset tests pass offline
- [x] `cargo test -p ancora-preset` runs without network in CI
- [x] Compliance notes published: `docs/preset/government-preset-compliance.md`
- [x] Security review completed: `docs/milestone/security-review-notes.md`
- [x] Red-team harness tested against injection patterns

## What is included

| Capability | Included | Justification |
|---|---|---|
| Planning | yes | Local task decomposition |
| Memory | yes | In-process episodic and semantic stores |
| Guardrails | yes | Required for high-security environments |
| Reasoning | yes | Citation and fact-checking, fully local |
| LongHorizon | yes | Checkpoint/restart, no external state |
| BehaviorEval | yes | Local evaluation metrics |

## What is excluded

| Capability | Excluded reason |
|---|---|
| Routing | Implies remote model dispatch |
| ToolSynthesis | May call external registries |
| Skills | May load remote skill descriptors |
| Coordination | May require cross-process communication |
| CostControl | Implies external cost metering API |

## Deployment guide

1. Deploy `ancora-preset` and its dependencies in an air-gapped environment.
2. Construct the preset with the correct zone identifier:
   ```rust
   let preset = government_compliant("your-zone-id");
   ```
3. Assemble and pass to your orchestrator:
   ```rust
   let spec = assemble(&preset).expect("always valid");
   ```
4. Verify `spec.system_prompt` contains `air_gap:required` and `locked:true`
   before execution.
5. Run `cargo test -p ancora-preset` in the target environment to confirm
   all 75 tests pass.
