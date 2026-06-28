# Advanced Feature Matrix

## Language support

| Feature | Rust | Go | Python | TypeScript | .NET | Java |
|---|---|---|---|---|---|---|
| Planning | full | parity | reference | reference | reference | reference |
| Reflection | full | parity | reference | reference | reference | reference |
| Routing | full | parity | reference | reference | reference | reference |
| Memory | full | parity | - | - | - | - |
| Tool synthesis | full | - | - | - | - | - |
| Skills | full | - | - | - | - | - |
| Long-horizon | full | - | - | - | - | - |
| Coordination | full | partial | - | - | - | - |
| Guardrails | full | partial | - | - | - | - |
| Reasoning | full | partial | - | - | - | - |
| Cost control | full | - | - | - | - | - |
| Behavior evals | full | - | - | - | - | - |
| Presets | full | - | - | - | - | - |
| Red-team | full | - | - | - | - | - |
| Benchmarks | full | - | - | - | - | - |

**full** = implemented and tested  
**parity** = canonical values validated  
**partial** = core metric validated, wrappers pending  
**reference** = use canonical constants from `ts_dotnet_java_batch.rs`

## Preset coverage

| Preset | Air-gap | Locked | Government-ready |
|---|---|---|---|
| research-assistant | no | no | no |
| coding-agent | no | no | no |
| customer-support | no | no | no |
| data-analysis | no | no | no |
| government-compliant | yes | yes | yes |

## Crate graph (dependency direction: left depends on right)

```
ancora-preset -> ancora-orchestrate, ancora-guard, ancora-reason, ...
ancora-advdet -> all 9 advanced crates
ancora-advpar -> all 9 advanced crates
ancora-redteam -> ancora-guard
ancora-advbench -> all 9 advanced crates
```
