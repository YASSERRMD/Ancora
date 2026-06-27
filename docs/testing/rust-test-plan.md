# Rust Test Plan

This document describes the Ancora Rust test suite structure, how to run tests, and what each category covers.

## Overview

The Rust SDK tests live entirely in `crates/ancora-core/tests/` and `crates/ancora-core/benches/`. All tests run offline: no live HTTP calls, no live vector stores, no live MCP servers. Network-dependent behavior is validated with pre-recorded journal fixtures embedded in the tests.

## Test Categories

### Unit tests (`crates/ancora-core/tests/`)

| File | Description |
|------|-------------|
| `journal_ordering_tests.rs` | MemoryStore append ordering and seq assignment |
| `journal_replay_tests.rs` | `replay_events()` correctness across status transitions |
| `run_state_machine_tests.rs` | All legal and illegal `RunStatus::transition()` combinations |
| `activity_concurrency_tests.rs` | `record_or_replay()` under concurrent writes |
| `replay_divergence_tests.rs` | `detect_divergence()` for observed vs expected event sequences |
| `idempotency_keys_tests.rs` | `WriteActivity::new()` and `write_once()` correctness |
| `retry_classification_tests.rs` | `classify()` and `run_with_retry()` for all error variants |
| `output_repair_tests.rs` | `validate_output()`, `validate_with_repair()`, `repair_prompt()` |
| `graph_validation_tests.rs` | `Graph::validate()` for cycles, missing nodes, empty graphs |
| `model_routing_tests.rs` | `ModelRouter` binding, override precedence, `bind_small()` |
| `suspend_resume_tests.rs` | `SuspendedRun` JSON round-trip and `RunOutcome` variants |
| `cancel_tests.rs` | `cancellation_pair()` pair semantics |
| `cost_aggregation_tests.rs` | `CostTracker` record and summary correctness |
| `executor_tests.rs` | `GraphExecutor` and `NodeExecutor` trait surface |
| `stream_tests.rs` | `open_stream()`, `emit_tokens()`, `StreamEvent` ordering |
| `verifier_consensus_tests.rs` | `VerifierNode` trait and majority-vote consensus |
| `journal_mask_extended_tests.rs` | `mask_events()` and `assert_structurally_equal()` |
| `error_code_tests.rs` | `AncoraError::error_code()` round-trips |
| `checkpoint_store_tests.rs` | `CheckpointStore::save()` and `load_checkpoint()` |
| `conformance_marker_tests.rs` | Cancel, error replay, and store clone sharing |

### End-to-end tests (offline fixtures)

| File | Description |
|------|-------------|
| `e2e_single_agent_tests.rs` | Single-node graph from start to completion |
| `e2e_multi_agent_verifier_tests.rs` | Two-node agent+verifier graph with consensus |
| `e2e_human_in_loop_tests.rs` | Suspend-on-human-input and resume flow |
| `e2e_rag_tests.rs` | Retrieval + generation pipeline with fixture chunks |
| `e2e_mcp_tool_use_tests.rs` | MCP tool call followed by LLM generation |
| `e2e_provider_mapping_tests.rs` | ModelRouter across Anthropic, OpenAI, Gemini, Mistral, DeepSeek |
| `e2e_vector_conformance_tests.rs` | Retrieval journals across LanceDB, pgvector, Qdrant |
| `e2e_structured_output_tests.rs` | JSON schema validation and output repair pipeline |
| `e2e_policy_enforcement_tests.rs` | Terminal-error enforcement for residency, max-steps, cancel |
| `e2e_graph_chain_tests.rs` | Linear chain and branching graph validation and replay |
| `e2e_cancellation_tests.rs` | CancellationToken propagation and journal correctness |
| `e2e_error_recovery_tests.rs` | Exponential backoff and transient error recovery |

### Conformance suite

| File | Description |
|------|-------------|
| `conformance_suite_tests.rs` | Drives all `ConformanceScenario` constants; validates Rust journals |
| `xlang_journal_equality.rs` | Asserts masked journals from different bindings are byte-equal |

### Reliability (chaos) tests

| File | Description |
|------|-------------|
| `chaos_kill_resume.rs` | Kill at various journal points; resume must complete exactly-once |
| `chaos_store_failures.rs` | Injected store faults; validates error isolation and recovery |
| `chaos_duplicate_effects.rs` | Zero duplicate side effects across multiple crash-resume cycles |
| `idempotency_chaos_tests.rs` | Property test: idempotency holds under random crash positions |
| `rate_limit_storm_tests.rs` | Burst 429/503 errors; retry budget and convergence |
| `memory_stability_tests.rs` | 1000-run and 500-activity stability; no unbounded growth |

### Property tests

| File | Description |
|------|-------------|
| `property_tests.rs` | Deterministic replay for random run IDs; activity key ordering |
| `wire_fuzz_tests.rs` | Arbitrary byte sequences decoded as `JournalEvent` must not panic |

### Security tests

| File | Description |
|------|-------------|
| `security_tool_poisoning_tests.rs` | Prompt-injection payloads treated as opaque data |
| `security_mcp_auth_tests.rs` | Unauthenticated MCP calls recorded for audit; auth check |
| `security_egress_tests.rs` | Full core API surface executes without opening any sockets |

## How to Run

### Run all tests

```bash
cargo test -p ancora-core
```

### Run a specific test file

```bash
cargo test -p ancora-core --test e2e_single_agent_tests
```

### Run a specific test by name

```bash
cargo test -p ancora-core single_agent_e2e_run_completes_successfully
```

### Run property tests (proptest)

Property tests are included in `cargo test`. To run more proptest cases:

```bash
PROPTEST_CASES=1000 cargo test -p ancora-core --test property_tests
```

### Run benchmarks

```bash
cargo bench -p ancora-core
```

To run a specific benchmark group:

```bash
cargo bench -p ancora-core -- replay_overhead
```

Benchmark results are written to `target/criterion/`.

### Coverage

Use the cargo alias defined in `.cargo/config.toml`:

```bash
cargo coverage
```

This requires `llvm-cov` (`cargo install cargo-llvm-cov`). The coverage gate is enforced by `codecov.yml`: 80% project coverage, 70% patch coverage.

## Test conventions

- All tests are offline by default. Tests that require live infrastructure are gated with `#[ignore]`.
- `MemoryStore` is the preferred backing store for tests. It is `Clone + Send + Sync` and shares internal state across clones.
- Journal events use `recorded_at_ns: (seq * 1_000_000) as i64` to produce monotonically increasing nanosecond timestamps.
- `ActivityRecordedEvent` requires `replayed: bool` field; set `replayed: false` for new recordings.
- `RetryPolicy` uses `jitter: f64` (not a bool). Set to `0.0` in tests to remove randomness.
- `ErrorEvent` fields are `code: String`, `message: String`, `detail: String`.

## Adding a new test

1. Create `crates/ancora-core/tests/<category>_tests.rs`.
2. Import only from `ancora_core` and `ancora_proto`.
3. Verify it compiles offline: `cargo test -p ancora-core --test <name> -- --list`.
4. Add it to this table under the appropriate category.
