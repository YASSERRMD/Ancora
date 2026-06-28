# Benchmark Report

This document summarises the performance benchmarks included in the Ancora test suite.
All benchmarks are offline and deterministic -- no live network calls, no external services.

## Benchmark Files

| File | Scenario | Budget |
|------|----------|--------|
| `bench_journal_write.rs` | 100k journal entries | 2000ms |
| `bench_replay_speed.rs` | 10k replay events | 500ms |
| `bench_vector_search.rs` | cosine search 5k chunks | 200ms |
| `bench_cost_computation.rs` | 1M cost calculations | 100ms |
| `bench_json_serialise.rs` | 50k JSON round-trips | 1000ms |
| `bench_otel_spans.rs` | 200k OTel span builds | 1000ms |
| `bench_a2a_envelope.rs` | 500k A2A envelopes | 500ms |
| `bench_parallel_join.rs` | 10k joins with 8 branches | 200ms |
| `bench_policy_check.rs` | 2M policy evaluations | 500ms |
| `bench_token_stream.rs` | 1M streaming tokens | 200ms |
| `bench_vector_insert.rs` | 20k vector inserts | 300ms |
| `bench_divergence_check.rs` | 100k divergence comparisons | 500ms |
| `bench_concurrent_journals.rs` | 50k atomic journal ops | 300ms |
| `bench_memory_tier.rs` | 5M tier lookups | 500ms |
| `bench_structured_output.rs` | 500k output validations | 300ms |
| `bench_checkpoint.rs` | 100k checkpoint round-trips | 500ms |
| `bench_event_dispatch.rs` | 5M event dispatches | 500ms |
| `bench_hil_gate.rs` | 2M HIL gate evaluations | 300ms |

## Design Principles

- All budgets are wall-clock milliseconds measured with `std::time::Instant`.
- Every benchmark fails fast if it exceeds its budget via `assert!`.
- Benchmarks are pure computation -- no I/O, no threads, no allocation beyond in-test buffers.
- Constants are named `*_BENCH_N` and `*_BENCH_MS` for easy tuning.

## Coverage

These 18 benchmark files exercise every major hot path:

- **Journal**: write throughput, replay speed, concurrent access.
- **Vector store**: search and insert.
- **Cost model**: single-function throughput at 1M calls.
- **Serialisation**: JSON and A2A envelope encoding.
- **OTel**: span construction.
- **Orchestration**: parallel joins, event dispatch, HIL gating.
- **Policy**: model allowlist and cost ceiling checks.
- **Token streaming**: LCG-based linear token scan.
- **Memory tiers**: hot/warm/cold routing.
- **Structured output**: field presence validation.
- **Durability**: checkpoint save and restore.
- **Replay correctness**: detect_divergence comparisons.
