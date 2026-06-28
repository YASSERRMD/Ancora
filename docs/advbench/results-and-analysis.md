# Advanced Benchmark Results and Analysis

## Capability overhead summary

| Capability | Typical elapsed | Token units | Quality |
|---|---|---|---|
| planner | < 200 ms (1,000 ops) | 100 (step count) | 0.75 |
| reflection | < 10 ms (10,000 ops) | - | 1.0 (grew) |
| routing | < 10 ms (10,000 ops) | 10,000 | 0.8 |
| optimization | < 200 ms (20 ops on 500 steps) | 500 | 0.8 |
| memory_consolidation | < 50 ms (100 consolidations) | promoted count |
| coordination | < 10 ms (1,000 records) | 1,000 |
| guardrail | < 10 ms (1,000 checks) | blocked count |
| reasoning | < 10 ms (500 citations) | 500 |
| lh_checkpoint | < 20 ms (500 tick loop) | 50 (every 10 ticks) |
| skills_jit | < 20 ms (200 loads) | 200 |

## Analysis

### Planning and optimization

Planning quality is O(n x m) where n = expected steps and m = actual steps
due to the `contains` scan.  For production use with large step counts (>200),
consider pre-sorting steps and using binary search.

### Routing

Routing overhead is negligible.  The 10,000-op loop runs in under 10 ms,
making routing suitable as a per-request filter without measurable latency
impact.

### Memory consolidation

The consolidation job involves summarization + promotion + dedup + forgetting.
Most overhead is in the episodic promotion scan (O(n)).  With 50 entries, 100
consolidation passes complete in under 50 ms.

### Guardrails

Injection detection is O(n x p) where n = input length and p = pattern count
(4 patterns).  At 1,000 checks it runs in under 10 ms.  GuardrailJournal
records all decisions without significant overhead.

### Long-horizon checkpoints

CheckpointCadence is O(1) per tick.  500 tick evaluations with 50 checkpoint
snapshots complete in under 20 ms.

## Cost-quality tradeoff

See [cost-quality-tradeoff.md](cost-quality-tradeoff.md) for the full table.
