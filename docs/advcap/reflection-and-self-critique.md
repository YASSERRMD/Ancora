# Reflection and Self-critique

Reflection is the capability for an agent to evaluate and improve its own output.
ancora-ageval's `ReflectionMetric` measures whether a second-pass output improved
over the first.

## Usage

```rust
use ancora_ageval::ReflectionMetric;

let before = "The answer is 42.";
let after = "The answer is 42. This follows from the constraint that X = 6 * 7.";

let score = ReflectionMetric::score(before, after);
// score = 1.0 (after is longer and different = improvement)
```

## Scoring Rules

| Condition | Score |
|---|---|
| `after == before` | 0.0 (no change) |
| `after.len() > before.len()` | 1.0 (improved) |
| `after != before && after.len() <= before.len()` | 0.5 (changed, not longer) |

## Integration with Orchestration

Run each agent task twice:

1. First pass: produce initial output
2. Reflection pass: produce refined output
3. Score: `ReflectionMetric::score(first, refined)`

Store scores in `EvalReport` to track reflection quality over time.

## Tip

Reflection is most valuable when the first pass was time-constrained. Combine
with `AbstentionPolicy` from ancora-reason: if the reflection score is 0.0, the
agent may need to abstain rather than repeat the same answer.
