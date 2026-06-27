# Multi-agent Graphs (Rust)

## Defining a graph

```rust
use ancora_core::{
    AgentSpec, GraphSpec, GraphNode, GraphEdge, Runtime, RunEvent,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;

    let researcher = AgentSpec::builder()
        .model("llama3")
        .instructions("Research the topic and summarise findings.")
        .build();

    let writer = AgentSpec::builder()
        .model("llama3")
        .instructions("Turn research notes into a polished paragraph.")
        .build();

    let graph = GraphSpec {
        nodes: vec![
            GraphNode { id: "researcher".into(), spec: researcher },
            GraphNode { id: "writer".into(), spec: writer },
        ],
        edges: vec![
            GraphEdge { from: "researcher".into(), to: "writer".into() },
        ],
        entry: "researcher".into(),
    };

    let mut run = rt.run_graph(&graph, "Explain Rust ownership").await?;

    while let Some(ev) = run.next().await? {
        if let RunEvent::Completed { output } = ev {
            println!("{}", output);
        }
    }
    Ok(())
}
```

## Parallel branches

Add multiple edges from an `entry` node to fan out work:

```rust
edges: vec![
    GraphEdge { from: "planner".into(), to: "branch_a".into() },
    GraphEdge { from: "planner".into(), to: "branch_b".into() },
    GraphEdge { from: "branch_a".into(), to: "merger".into() },
    GraphEdge { from: "branch_b".into(), to: "merger".into() },
],
```

## Passing context between nodes

The output of each node becomes the input prompt for its successors.
For structured hand-off, format the output as JSON and parse it in the
next node's handler tool.

## See also

- [Verifier pattern](verifier.md)
- [Tools](tools.md)
