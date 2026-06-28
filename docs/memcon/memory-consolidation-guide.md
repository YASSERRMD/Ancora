# Memory Consolidation Guide

ancora-memcon keeps long-running agent memory compact and useful through
summarization, salience scoring, episodic-to-semantic promotion, deduplication,
and forgetting.

## Summarization

`ConversationSummarizer` collapses older turns into a rolling summary while
keeping the `keep_last_n` most recent turns verbatim.

```rust
let policy = SummarizationPolicy::new(10, 3);
let summarizer = ConversationSummarizer::new(policy);
let result = summarizer.summarize(&turns);
// result.summary: collapsed text of dropped turns
// result.kept: last 3 turns unchanged
```

## Salience Scoring

`SalienceScorer::default_weights()` scores items by importance, recency, and
access frequency. Higher score means higher retention priority.

## Episodic to Semantic Promotion

`EpisodicToSemanticPromoter::new(min_occurrences)` promotes facts that recur
across episodes into long-term semantic memory.

## Deduplication

`Deduplicator::dedup_by_key` keeps the first occurrence of each key, dropping
later duplicates.

## Forgetting Policy

`ForgettingPolicy::new(min_salience, max_age_secs)` drops items that are too
old or score below the salience threshold.

## Consolidation Job

`ConsolidationJob::run` executes all steps in a single pass and journals every
event for deterministic replay.

## Token Budget

`TokenBudget::estimate_tokens` uses a 1 token per 4 characters heuristic to
track footprint reduction after consolidation.
