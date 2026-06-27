# Ancora Rust Examples

Runnable example tests for the Ancora Rust crates.
All tests run offline -- no live API keys, no network calls.

## Run

```bash
cargo test -p ancora-examples
```

Run a single example:

```bash
cargo test -p ancora-examples --test single_agent
```

## Examples

| Test file | Description |
|-----------|-------------|
| `single_agent` | Run lifecycle: `Pending -> Running -> Completed` |
| `structured_output` | Derive JSON Schema from Rust structs, validate round-trips |
| `multi_agent_verifier` | Concurrent runs with distinct IDs via `std::thread` |
| `human_in_loop` | `HumanDecisionRequested` and `HumanDecisionReceived` events |
| `streaming_chat` | Token events accumulated in seq order from `MemoryStore` |
| `rag_lancedb` | Offline keyword retrieval standing in for LanceDB similarity |
| `mcp_tool` | `ToolSpec` definition, `AgentSpec` wiring, local dispatch |
| `glm_llama_edge` | GLM model variant specs and distinct run IDs |
| `durable_restart` | `RunJournal` + `MemoryStore` for event replay |
| `cost_otel` | `Span` timing and `TokenEstimator` (4 chars / token) |

## Shared helpers

`src/lib.rs` provides `RunJournal`, `Span`, `TokenEstimator`, `Passage`,
and `keyword_retrieve` utilities used across the example tests.
