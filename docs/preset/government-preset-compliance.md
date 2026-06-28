# Government Preset Compliance Notes

## Purpose

The `government_compliant` preset is designed for deployment in sovereign,
classified, or air-gapped environments where:

- All compute must remain within a defined geographic zone
- No external network calls are permitted
- The capability set is fixed and cannot be expanded at runtime

## Constraints enforced

| Constraint | Value | Reason |
|---|---|---|
| Air-gap | `AirGapPolicy::Required` | No outbound connections permitted |
| Residency | `ResidencyConstraint::Zone(zone)` | Data stays in the named zone |
| Locked | `true` | Capability set is immutable |
| Routing | excluded | Routing implies remote model dispatch |

## Using the preset

```rust
use ancora_preset::{assemble, government_compliant};

let preset = government_compliant("us-gov-east-1");
let spec = assemble(&preset).expect("government preset assembles");

// Confirm constraints in the assembled spec
assert!(spec.system_prompt.contains("air_gap:required"));
assert!(spec.system_prompt.contains("residency_zone:us-gov-east-1"));
assert!(spec.system_prompt.contains("locked:true"));
```

## What is and is not included

| Capability | Included | Rationale |
|---|---|---|
| Planning | yes | Local task decomposition, no network |
| Memory | yes | In-process episodic and semantic stores |
| Guardrails | yes | Critical for high-security environments |
| Reasoning | yes | Citation and fact-checking, local |
| LongHorizon | yes | Checkpoint/restart, no external state |
| BehaviorEval | yes | Local evaluation metrics |
| Routing | no | Requires remote model dispatch |
| ToolSynthesis | no | May call external registries |
| Skills | no | May load remote skill descriptors |
| Coordination | no | May require cross-process communication |
| CostControl | no | Implies cost metering via external API |

## Airgap validation

The validator rejects any preset that combines `AirGapPolicy::Required` with
the `Routing` capability.  This prevents accidental misconfiguration:

```rust
use ancora_preset::{validate, AirGapPolicy, Capability, PresetDescriptor, ValidationError};

let bad = PresetDescriptor::new("bad", "conflicting config")
    .with_capability(Capability::Planning)
    .with_capability(Capability::Routing)
    .with_air_gap(AirGapPolicy::Required);

let errs = validate(&bad).unwrap_err();
assert!(errs.contains(&ValidationError::AirGapConflictsWithRouting));
```

## Compliance checklist

Before deploying the government preset in a regulated environment:

- [ ] Verify that `residency_zone` matches your approved deployment region
- [ ] Confirm no capability in `spec.tools` requires external I/O
- [ ] Confirm the orchestrator enforces the `locked:true` flag
- [ ] Run `cargo test -p ancora-preset` to verify all 71 tests pass offline
- [ ] Audit `spec.system_prompt` for unexpected lines before handoff to executor

## Zone identifiers

Zone strings are opaque to Ancora.  Use whatever identifier your infrastructure
requires (AWS GovCloud region, DISA zone label, etc.).  The value is encoded
verbatim into the system prompt as `residency_zone:<value>`.
