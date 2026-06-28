# Eval Suite Catalog

ancora-evallib ships a catalog of nine reusable offline eval suites.
Each suite is independent and needs no network access.

## Suites

| Suite | Module | Cases | What it measures |
|---|---|---|---|
| Tool-use | `tool_use` | 3 | Correct tool selection given a task description |
| RAG faithfulness | `rag_faithfulness` | 3 | Answer grounded in retrieved context |
| Coordination | `coordination` | 1 | Multi-agent result assembly |
| Reasoning | `reasoning` | 4 | Arithmetic, logical, and causal reasoning |
| Safety | `safety` | 4 | Refusal of harmful requests |
| Routing | `routing` | 3 | Request dispatched to the right model tier |
| Long-context | `long_context` | 2 | Fact retrieval from large documents |
| Multilingual | `multilingual` | 4 | Understanding and response in multiple languages |
| Cost-efficiency | `cost_efficiency` | 3 | Quality within token budget |

## Running all suites

```rust
use ancora_evallib::runner::run_offline_eval;

fn main() {
    let report = run_offline_eval();
    println!("Pass rate: {:.1}%", report.overall_pass_rate() * 100.0);
}
```

## Adding a new suite

1. Create `src/<suite_name>.rs` with a `*Suite` struct and a `run_all() -> (usize, usize)` method.
2. Declare it as `pub mod <suite_name>;` in `src/lib.rs`.
3. Call it from `runner::run_offline_eval()`.
4. Add test cases in `src/tests/test_<suite_name>.rs`.
