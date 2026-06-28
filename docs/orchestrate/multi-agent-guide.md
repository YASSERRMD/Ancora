# Ancora Multi-Agent Orchestration Guide

The `ancora-orchestrate` crate provides the infrastructure for spawning and coordinating multiple agents.

## Agent roles

| Role | Purpose |
|------|---------|
| Orchestrator | Decomposes a goal into tasks and spawns subagents |
| Subagent | Executes a focused task with specific tools |
| Critic | Reviews subagent outputs for quality or safety |
| Synthesizer | Merges results from multiple subagents |

## Task graph

```rust
use ancora_orchestrate::TaskGraph;

let mut graph = TaskGraph::new();
graph.add_task("search", vec![]);
graph.add_task("summarize", vec!["search".to_string()]);
graph.add_task("critique", vec!["summarize".to_string()]);

assert!(!graph.has_cycle());

// Dispatch loop
while !graph.all_complete() {
    for task_id in graph.ready_tasks() {
        graph.mark_running(task_id);
        // dispatch...
        graph.mark_completed(task_id);
    }
}
```

## Spawn tracking

```rust
use ancora_orchestrate::{SpawnTracker, AgentTask};
use serde_json::json;

let mut tracker = SpawnTracker::new();
tracker.spawn("orchestrator", AgentTask::new("t1", "subagent-1", json!({"query": "foo"})), now);
println!("Spawned: {}", tracker.total_spawned());
```

## Result aggregation

```rust
use ancora_orchestrate::{ResultAggregator, AgentResult};

let mut agg = ResultAggregator::new();
agg.record(AgentResult { task_id: "t1".into(), agent_id: "a1".into(), output: json!("result"), success: true });
let merged = agg.merge_outputs(&["t1", "t2"]);
```
