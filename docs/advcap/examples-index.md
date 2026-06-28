# Examples Index

All runnable examples live in `crates/ancora-examples/src/`.

## Core Examples

| File | Demonstrates |
|---|---|
| `fan_out_tasks.rs` | `fan_out` with ancora-orchestrate |
| `tool_use.rs` | Tool registration and dispatch |
| `streaming.rs` | Partial result streaming |
| `memory_consolidation.rs` | ancora-memcon consolidation pipeline |
| `tool_synthesis.rs` | ancora-toolsynth dynamic tool generation |
| `skill_loader.rs` | ancora-skills JIT loading |
| `long_horizon.rs` | ancora-lh background run lifecycle |
| `coordination.rs` | ancora-coord blackboard and bidding |
| `guardrail_pipeline.rs` | ancora-guard policy composition |

## Advanced Capability Examples

| File | Demonstrates |
|---|---|
| `verified_reasoning.rs` | Full ancora-reason pipeline with citations |
| `agent_eval_harness.rs` | All 7 ancora-ageval metrics with baseline and report |
| `advanced_combined.rs` | ancora-adv-integration: all 9 crates combined |

## Running Examples

```bash
cargo run -p ancora-examples --example fan_out_tasks
cargo run -p ancora-examples --example verified_reasoning
cargo run -p ancora-examples --example advanced_combined
```

All examples are offline and network-free. They use in-memory stores and u64 ticks.

## Go SDK Examples

```bash
cd sdk/go/single-agent && go run .
cd sdk/go/multi-agent-verifier && go run .
cd sdk/go/structured-output && go run .
```
