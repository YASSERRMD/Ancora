# Advanced Capabilities Overview

## Scope

Phases 161-200 shipped the following advanced-capability crates, each offline
and deterministically tested:

| Crate | Phase | Capabilities |
|---|---|---|
| ancora-orchestrate | 161 | Planning, fan-out, task graph |
| ancora-memcon | 162 | Episodic memory, consolidation, token budget |
| ancora-toolsynth | 163 | Tool synthesis, SynthCache, audit |
| ancora-skills | 164 | Skill JIT loading, crew, journal |
| ancora-lh | 165 | Long-horizon checkpoints, wakeup, throttle |
| ancora-coord | 166 | Coordination, contract-net, deadlock detection |
| ancora-guard | 167 | Guardrails, injection detection, PII, policy |
| ancora-reason | 168 | Structured reasoning, citations, fact-check |
| ancora-ageval | 169 | 7-metric behavior evaluation framework |
| ancora-adv-integration | 170 | Cross-crate integration patterns |
| ancora-advdet | 195 | Determinism test suite (71 tests) |
| ancora-redteam | 196 | Red-team harness, adversarial scenarios |
| ancora-advpar | 197 | Cross-language parity (94 tests) |
| ancora-preset | 198 | Capability presets (75 tests) |
| ancora-advbench | 199 | Performance/cost benchmarks (45 tests) |

## Key design principles

- **Offline first**: all advanced crates run in-process with no network calls
- **Deterministic**: u64 tick timestamps, no wall-clock time, no RNG
- **Validated**: minimum 20 tests per phase, all suites green
- **Documented**: capability docs, red-team guide, government compliance notes
- **Benchmarked**: regression-gated overhead for all 10 capability areas

## Total test count

As of Phase 200, the advanced capability workspace contains over 400 tests
across 15 crates.
