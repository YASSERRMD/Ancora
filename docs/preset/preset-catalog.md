# Ancora Capability Preset Catalog

A preset bundles a curated set of capabilities and compliance constraints into
a named, validated descriptor.  Call `assemble()` to turn a preset into an
`AgentSpec` you can pass to the orchestrator.

## Available presets

| Name | Function | Key capabilities | Locked | Air-gap |
|---|---|---|---|---|
| research-assistant | `research_assistant()` | Memory, Reasoning, LongHorizon, Planning, Reflection, BehaviorEval | no | none |
| coding-agent | `coding_agent()` | Planning, ToolSynthesis, Skills, Guardrails, Reflection, CostControl | no | none |
| customer-support | `customer_support()` | Routing, Guardrails, Memory, Coordination, CostControl | no | none |
| data-analysis | `data_analysis()` | Planning, Reasoning, Memory, ToolSynthesis, BehaviorEval, CostControl | no | none |
| government-compliant | `government_compliant(zone)` | Planning, Memory, Guardrails, Reasoning, LongHorizon, BehaviorEval | yes | required |

## Quick start

```rust
use ancora_preset::{assemble, research_assistant};

let preset = research_assistant();
let spec = assemble(&preset).expect("preset is always valid");
// spec is now an AgentSpec ready to hand to the orchestrator
```

## Validation

Every preset is validated before assembly.  The `validate()` function returns
an accumulated list of `ValidationError` values:

| Error | Cause |
|---|---|
| `EmptyName` | `name` is empty or whitespace only |
| `EmptyDescription` | `description` is empty or whitespace only |
| `NoCapabilities` | no capabilities listed |
| `AirGapConflictsWithRouting` | `air_gap: Required` combined with `Routing` capability |
| `EmptyOverrideKey` | an override pair has an empty key |

`assemble()` calls `validate()` internally and returns `Err(AssemblyError)`
if validation fails.

## Preset capabilities reference

| Capability token | System-prompt key |
|---|---|
| `Planning` | `capability:planning` |
| `Reflection` | `capability:reflection` |
| `Routing` | `capability:routing` |
| `Memory` | `capability:memory` |
| `ToolSynthesis` | `capability:tool_synthesis` |
| `Skills` | `capability:skills` |
| `LongHorizon` | `capability:long_horizon` |
| `Coordination` | `capability:coordination` |
| `Guardrails` | `capability:guardrails` |
| `Reasoning` | `capability:reasoning` |
| `CostControl` | `capability:cost_control` |
| `BehaviorEval` | `capability:behavior_eval` |
