# Memory Consolidation

ancora-memcon provides primitives for compressing and reorganizing agent memory
between turns: summarization, salience scoring, episodic-to-semantic promotion,
deduplication, forgetting, token budgeting, and a deterministic journal.

## Consolidation Pipeline

```rust
use ancora_memcon::{ConsolidationJob, ConsolidationJournal};

let job = ConsolidationJob { summarizer, scorer, promoter, forgetting };
let output = job.run(&turns, salience_items, &episodic, tick, &mut journal);

println!("Summary: {:?}", output.summary);
println!("Promoted: {} semantic entries", output.promoted.len());
println!("Retained: {} items", output.retained.len());
```

## Key Primitives

| Primitive | Purpose |
|---|---|
| `ConversationSummarizer` | Collapse old turns into a summary |
| `SalienceScorer` | Score items by importance, recency, frequency |
| `EpisodicToSemanticPromoter` | Promote recurring episodes to semantic memory |
| `Deduplicator` | Remove items with duplicate keys |
| `ForgettingPolicy` | Drop items below salience threshold or past max age |
| `TokenBudget` | Ensure retained content fits token budget |
| `ConsolidationJournal` | Append-only record of every consolidation step |

## Eval

Track retention quality with `MemoryMetric`:

```rust
let score = MemoryMetric::score(output.retained.len(), total_items);
```

## Determinism

All consolidation operations are deterministic given the same tick value and
salience weights. The `ConsolidationJournal` enables full replay.
