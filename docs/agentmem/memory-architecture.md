# Memory Architecture

Ancora agents use a two-tier memory system:

## Tier 1: Working memory (short-term)

`WorkingMemory` is a ring buffer scoped to the current agent turn. It holds:
- Intermediate reasoning steps
- Tool call results not yet consolidated
- Scratch-pad facts the agent is tracking within this turn

Capacity: 32-64 items. Cleared at the end of each turn unless promoted to long-term memory.

## Tier 2: Long-term memory (persistent)

`MemoryStore` holds facts, preferences, instructions, and context entries across turns. Entries are scored by:

```
score = importance * recency_weight * (1 + log(access_count + 1))
```

When the store is full, the lowest-scored entry is evicted.

## Context window budget

`ContextBudget` tracks token usage in the current conversation. When utilization approaches 80%, trigger conversation compression via `ConversationCompressor` before the next model call.

## Retrieval pipeline

1. `MemoryStore::top_k(100, now)` - get the most relevant candidates
2. `KeywordRetriever::retrieve(&candidates, query, 5)` - filter by query
3. Inject retrieved memories into the system prompt as bullet points

## Forgetting

`prune_by_age(store, 7 * 24 * 3600, now)` removes entries not accessed in 7 days. Run weekly during maintenance windows.
