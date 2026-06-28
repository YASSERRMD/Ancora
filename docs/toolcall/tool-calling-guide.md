# Ancora Advanced Tool-Calling Guide

The `ancora-toolcall` crate provides the infrastructure for multi-step agent tool dispatch.

## Registering tools

```rust
use ancora_toolcall::{ToolRegistry, ToolDef};

let mut registry = ToolRegistry::new();
registry.register(ToolDef::new("web_search", "Search the web").with_timeout(10_000));
registry.register(ToolDef::new("code_exec", "Execute Python code").async_tool());
```

## Selecting tools for a model turn

```rust
use ancora_toolcall::ToolSelector;

let tools: Vec<&ToolDef> = registry.names().iter()
    .filter_map(|n| registry.get(n))
    .collect();

let selector = ToolSelector::new(5);
let relevant = selector.select(&tools, "search"); // returns tools matching "search"
```

## Dispatching tool calls in parallel

```rust
use ancora_toolcall::{ParallelDispatcher, ToolCall, ToolResult};
use serde_json::json;

let calls = vec![
    ToolCall::new("c1", "web_search", json!({"q": "Rust async"})),
    ToolCall::new("c2", "web_search", json!({"q": "Tokio runtime"})),
];

let dispatcher = ParallelDispatcher::new(8);
let results = dispatcher.execute(calls, |call| {
    // dispatch to actual tool implementation
    Ok(ToolResult::ok(&call.call_id, &call.tool_name, json!("result"), 50))
})?;
```

## Merging results back into context

```rust
use ancora_toolcall::{merge_results, results_to_messages};

let merged_context = merge_results(&results);
let messages = results_to_messages(&results);
// feed messages into the next model turn
```

## Dependency graph

```rust
use ancora_toolcall::CallGraph;

let mut graph = CallGraph::new();
graph.add_dependency("summarize", "web_search"); // summarize depends on search
assert!(!graph.can_run_parallel("web_search", "summarize"));
assert!(!graph.has_cycle());
```
