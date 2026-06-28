# Cost-Quality Tradeoff Tables

## Planning

| Step count | Iterations | Elapsed (typical) | Quality gain |
|---|---|---|---|
| 10 | 1,000 | < 1 ms | proportional to match ratio |
| 100 | 1,000 | < 200 ms | 0.75 at 75% match |
| 500 | 20 | < 200 ms | 0.80 at 80% match |

**Trade-off:** larger step sets improve decomposition quality but increase
matching cost quadratically.  For most agents, 20-50 steps is optimal.

## Reflection

| Input sizes | Iterations | Elapsed (typical) | Quality |
|---|---|---|---|
| small (10 chars each) | 10,000 | < 1 ms | 1.0 or 0.5 |
| medium (200/300 chars) | 10,000 | < 10 ms | 1.0 |
| large (1 KB each) | 10,000 | < 10 ms | length comparison only |

**Trade-off:** reflection quality is binary (grew/shrunk/same).  Cost is
negligible.  The real cost is generation, not evaluation.

## Routing

| Quality | Cost (tokens) | Max (tokens) | Score |
|---|---|---|---|
| 0.9 | 0 | 1,000 | 0.95 |
| 0.9 | 300 | 1,000 | 0.80 |
| 0.9 | 600 | 1,000 | 0.65 |
| 0.9 | 1,000 | 1,000 | 0.45 |

**Trade-off:** high quality with high cost scores worse than lower quality
with low cost.  Prefer cheap models for high-volume routing.

## Memory consolidation

| Entry count | Min occurrences | Promoted | Elapsed (100 passes) |
|---|---|---|---|
| 50 | 1 | all | < 50 ms |
| 50 | 3 | 25% | < 50 ms |
| 50 | 5 | 10% | < 50 ms |

**Trade-off:** higher `min_occurrences` reduces the promoted set (saving
tokens downstream) with no change in consolidation speed.

## Guardrails

| Inputs | Blocked % | Elapsed |
|---|---|---|
| 1,000 (1 in 3 injections) | ~33% | < 10 ms |
| 1,000 (no injections) | 0% | < 10 ms |

**Trade-off:** guardrail latency is independent of block rate.

## Regression thresholds

| Capability | Baseline (ns) | CI gate (2x, ns) |
|---|---|---|
| planner | 200,000,000 | 400,000,000 |
| reflection | 10,000,000 | 20,000,000 |
| routing | 10,000,000 | 20,000,000 |
| optimization | 200,000,000 | 400,000,000 |
| memory_consolidation | 50,000,000 | 100,000,000 |
| coordination | 10,000,000 | 20,000,000 |
| guardrail | 10,000,000 | 20,000,000 |
| reasoning | 10,000,000 | 20,000,000 |
| lh_checkpoint | 20,000,000 | 40,000,000 |
| skills_jit | 20,000,000 | 40,000,000 |
