# Ancora Agent Memory Guide

The `ancora-agentmem` crate provides memory and context management for long-running agents.

## Memory entries

```rust
use ancora_agentmem::{MemoryEntry, MemoryKind, MemoryStore};

let mut store = MemoryStore::new(1000); // max 1000 entries
store.insert(
    MemoryEntry::new("m1", MemoryKind::Fact, "user prefers Rust over Python", 8, now),
    now,
);

// Retrieve by score (recency * importance * frequency)
let top = store.top_k(5, now);
```

## Context window budgeting

```rust
use ancora_agentmem::ContextBudget;

let mut budget = ContextBudget::new(128_000, 2_000); // 128k ctx, 2k system
budget.add_message(500); // user turn
budget.add_message(800); // assistant turn
println!("Utilization: {:.1}%", budget.utilization_pct());
```

## Conversation compression

```rust
use ancora_agentmem::{ConversationCompressor, ConversationTurn};

let compressor = ConversationCompressor::new(200); // summary uses 200 tokens
let compressed = compressor.compress(turns, budget_limit, 4 /* keep last 4 */);
```

## Keyword retrieval

```rust
use ancora_agentmem::KeywordRetriever;

let all_memories: Vec<&MemoryEntry> = store.top_k(100, now);
let relevant = KeywordRetriever::retrieve(&all_memories, "rust async", 5);
// Use relevant memories to build the system prompt context
```
