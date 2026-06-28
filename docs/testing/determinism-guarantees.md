# Determinism Guarantees and Limits

Ancora guarantees that any run can be replayed from its journal and produce the same logical outcome -- the same event sequence, the same activity results, the same cost, and the same OTel trace structure -- regardless of when or how many times the replay runs.

## What is guaranteed

1. **Identical inputs, identical outputs.** Given the same journal events, `replay_events()` returns the same `RunStatus` and the same activity result sequence every time.
2. **Recorded activities never re-execute.** An `ActivityRecordedEvent` with `replayed=true` is used as-is. The original execution path (LLM call, tool call, vector retrieval) is not triggered again.
3. **Stable parallel join order.** When multiple activities run in parallel, their journal entries are ordered by `activity_key`. The join order is therefore stable and predictable.
4. **Stable map serialisation.** All JSON fields inside `input_json` and `result_json` are serialised using `BTreeMap`-ordered keys (sorted alphabetically) so that the same inputs always produce byte-identical JSON.
5. **Journaled time and randomness.** The `recorded_at_ns` field captures the wall-clock nanosecond timestamp at the moment of first execution. On replay this value is read from the journal; the system clock is never consulted. Any random seeds used inside a run must similarly be recorded as activity inputs.
6. **Replay across process restarts.** The journal is the only state that survives a process restart. A new process that loads the journal produces the same outcome.
7. **Replay across language bindings.** The journal format is language-agnostic proto/JSON. Any Ancora SDK (Rust, Go, Python, TypeScript, .NET, Java) that loads the same journal events arrives at the same structural outcome (same `kind` sequence, same `seq` values, same `run_id`). `result_json` and `output_json` may differ if the model's text output is not included in the determinism check.
8. **Divergence detection.** If the current activity-key sequence does not match the journal's expected sequence, `detect_divergence()` returns an error immediately. This catches code changes that alter execution paths without updating the journal.
9. **Cost reproduction.** Token counts are embedded in `result_json` during live execution. On replay the same counts are read back, so cost summaries are deterministic.
10. **OTel span reproduction.** `trace_id`, `span_id`, and `parent_span_id` are recorded in `input_json` / `result_json` at first execution. Replay reads them from the journal.
11. **Idempotency.** Running `replay_events()` twice on the same journal events returns the same result and does not modify the journal.
12. **Partial journal resume.** A journal that ends before `RunCompleted` represents an in-progress run. `replay_events()` returns a non-`Completed` status, allowing the caller to resume from the last recorded point.
13. **Corruption detection.** Journals with out-of-order `seq` values or duplicate `event_id`s are considered corrupt. Applications must validate these properties before replaying.
14. **Version migration.** When the schema for an activity's `input_json` changes, old journals can be migrated by transforming the stored JSON before replaying. After migration, the journal must still pass all determinism checks.

## What is NOT guaranteed

- **Model text output.** The text inside `result_json["text"]` and `output_json` comes from a language model and is non-deterministic at the API level (temperature > 0, sampling). Once recorded, it is replayed exactly. But across first-run executions the text may differ.
- **Wall-clock elapsed time.** `recorded_at_ns` captures when an event occurred. Replay does not re-introduce real delays; it processes events synchronously. Wall-clock profiling must be done on live runs.
- **Network latency.** Replay never issues network calls. Latency from LLM providers, vector stores, or MCP servers is not reproducible from the journal alone.
- **Operating-system randomness.** Any `getrandom`/`/dev/urandom` calls not captured as activity inputs are non-deterministic. Use journaled seeds for any randomness that must be reproducible.

## Test suite location

All determinism tests live in `crates/ancora-core/tests/det_*.rs` and run with:

```bash
cargo test -p ancora-core --test det_identical_inputs \
  --test det_no_re_execute \
  --test det_parallel_join \
  --test det_map_ordering \
  --test det_time_and_random \
  --test det_process_restart \
  --test det_cross_language_replay \
  --test det_divergence_code_change \
  --test det_divergence_schema_change \
  --test det_property_random_graphs \
  --test det_property_tool_sequences \
  --test det_large_journal_perf \
  --test det_partial_journal \
  --test det_corrupted_journal \
  --test det_version_migration \
  --test det_cost_replay \
  --test det_otel_replay \
  --test det_no_network_replay \
  --test det_idempotent_replay
```

All tests run offline -- no live HTTP calls, no live stores, no live MCP servers.
