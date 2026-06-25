# Migrating from LangGraph to Ancora

This guide covers the key conceptual shifts and mechanical steps for teams
moving from LangGraph (Python) to Ancora.

## Core model comparison

| LangGraph | Ancora | Notes |
|-----------|--------|-------|
| `StateGraph` | `Graph` | Ancora graphs are language-agnostic and serialised as protobuf |
| `add_node(name, fn)` | `Node { id, kind, spec }` | Node kinds are typed: `Agent`, `Tool`, `Human` |
| `add_edge(a, b)` | `Edge { from, to, condition }` | Conditions are optional JSON-path predicates |
| `add_conditional_edges` | `Edge { condition }` | Same concept, declarative JSON instead of Python callables |
| `graph.invoke(state)` | `agent.run(spec)` / `runtime.start_run(spec)` | Ancora returns an event stream, not a final state dict |
| `MemorySaver` | `SqliteStore` / `MemoryStore` | Ancora journals are append-only; replay is automatic |
| `StateAnnotation` | Agent `output_schema_json` | Output schema is a JSON Schema string attached to the spec |

## Node migration

### LangGraph node (Python)

```python
def my_node(state: State) -> dict:
    response = llm.invoke(state["messages"])
    return {"messages": [response]}

graph.add_node("my_node", my_node)
```

### Ancora equivalent (any language)

Define a `GraphSpec` with one `AgentNode`:

```json
{
  "id": "g-1",
  "entry_node": "my_node",
  "nodes": [{
    "id": "my_node",
    "kind": "Agent",
    "spec": {
      "name": "my_node",
      "model_id": "claude-3-5-haiku-20241022",
      "instructions": "You answer questions helpfully.",
      "max_steps": 5
    }
  }],
  "edges": []
}
```

## Tool migration

LangChain/LangGraph tools become Ancora tools via the `langchain_adapter`
module (Rust) or native tool registration in each language SDK:

```rust
// Rust
use ancora_tools::langchain_adapter::{LangchainTool, from_langchain};

let tool = LangchainTool::new("search", "searches the web", |query| {
    // existing search logic
    Ok(format!("results for: {}", query))
});
let ancora_tool = from_langchain(tool);
registry.register(ancora_tool);
```

```python
# Python SDK
from ancora import ToolSpec, Runtime

@runtime.tool(description="searches the web")
def search(query: str) -> str:
    return f"results for: {query}"
```

## State management

LangGraph uses a mutable `state` dict threaded through every node.  Ancora
uses an **event journal** instead:

- Each node emits `NodeEntered`, `ActivityRecorded`, and `NodeExited` events.
- Downstream nodes receive the prior node's output via the graph executor,
  not a shared dict.
- Cross-node data flow is explicit in the `AgentSpec.instructions` or via
  a tool that reads earlier outputs.

## Conditional routing

LangGraph:

```python
def route(state: State) -> str:
    if state["approved"]:
        return "approve_node"
    return "reject_node"

graph.add_conditional_edges("review", route, {"approve_node": "approve_node", "reject_node": "reject_node"})
```

Ancora equivalent in graph JSON:

```json
{
  "edges": [
    { "from": "review", "to": "approve_node", "condition": "$.approved == true" },
    { "from": "review", "to": "reject_node",  "condition": "$.approved == false" }
  ]
}
```

## Human-in-the-loop

LangGraph: `interrupt()` or `interrupt_before`/`interrupt_after` on a node.

Ancora: a `HumanNode` in the graph or `runtime.suspend(run_id, prompt)` from
inside a tool.  Resume with `runtime.resume(run_id, decision)`.  The journal
persists the suspension; no state is lost on process restart.

## Checkpointing

LangGraph checkpoints are per-step dict snapshots.  Ancora journals are
**event-sourced**: the full run is reconstructed by replaying journal events,
so replay is deterministic and crash recovery is free.

## Step-by-step migration checklist

1. List all LangGraph nodes and classify each as `Agent`, `Tool`, or `Human`.
2. Convert each node's state keys to an explicit JSON Schema output.
3. Replace `add_conditional_edges` with declarative `Edge` conditions.
4. Wrap existing LangChain tools with `from_langchain()` or the native SDK adapter.
5. Replace `MemorySaver` with `SqliteStore` or `PostgresStore`.
6. Replace `graph.invoke(state)` with `agent.run(spec).events()` and handle
   the event stream.
7. Replace `interrupt()` checkpoints with `HumanNode` or `runtime.suspend()`.
8. Run the Ancora conformance suite to verify the migrated graph.
