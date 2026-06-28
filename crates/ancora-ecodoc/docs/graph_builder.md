# Graph Builder

The graph builder API lets plugins define directed acyclic graphs (DAGs) of tasks.

## Creating a graph

```rust
use ancora_ecodoc::graph_builder::{TaskGraph, GraphNode};

let mut graph = TaskGraph::new();
graph.add_node(GraphNode { id: "fetch".into(), label: "Fetch data".into() })?;
graph.add_node(GraphNode { id: "process".into(), label: "Process data".into() })?;
graph.add_edge("fetch", "process")?;

assert!(!graph.has_cycle());
```

## Cycle detection

Always call `has_cycle()` before submitting the graph to the runtime.
The runtime will reject cyclic graphs at startup.
