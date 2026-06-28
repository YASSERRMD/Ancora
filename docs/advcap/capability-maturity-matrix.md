# Capability Maturity Matrix

This matrix describes the current maturity of each advanced Ancora capability.

## Maturity Levels

| Level | Label | Meaning |
|---|---|---|
| 1 | Prototype | API exists, basic tests, no stability guarantee |
| 2 | Stable | Tested with >20 assertions, API frozen, journal replay works |
| 3 | Production | Battle-tested, full eval integration, regression baselines set |

## Capability Matrix

| Crate | Capability | Level | Tests | Eval Metric |
|---|---|---|---|---|
| ancora-orchestrate | Fan-out planning | 3 | yes | `PlanningMetric` |
| ancora-orchestrate | Reflection / replanning | 3 | yes | `ReflectionMetric` |
| ancora-orchestrate | Learned routing | 3 | yes | `RoutingMetric` |
| ancora-memcon | Memory consolidation | 2 | yes | `MemoryMetric` |
| ancora-toolsynth | Tool synthesis | 2 | yes | n/a |
| ancora-skills | Skills / sub-agents | 2 | yes | `CoordinationMetric` |
| ancora-lh | Long-horizon lifecycle | 2 | yes | n/a |
| ancora-coord | Coordination / contracts | 2 | yes | `CoordinationMetric` |
| ancora-guard | Guardrails / audit | 3 | yes | `GuardrailMetric` |
| ancora-reason | Structured reasoning | 2 | yes | `ReasoningMetric` |
| ancora-ageval | Behavior evaluation | 2 | yes | all 7 metrics |

## Gaps

- ancora-reason: contradiction detection is O(n^2); needs optimization for >500 steps
- ancora-toolsynth: sandbox does not yet enforce memory limits
- ancora-lh: `ConsolidationJob` does not yet persist to disk; in-memory only
- ancora-coord: `Negotiation` supports only numeric proposals; no freeform types yet

## Roadmap to Production

To advance a capability from Level 2 to Level 3:
1. Add regression baselines via `BaselineStore` for all associated metrics
2. Wire into `ancora-adv-integration` tests
3. Document in `docs/advcap/` with working code examples
4. Pass `test_perf.rs` performance thresholds
