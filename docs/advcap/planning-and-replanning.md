# Planning and Replanning

ancora-orchestrate provides task decomposition and fan-out primitives for
structured planning pipelines.

## Core Types

- `TaskGraph` - DAG of agent tasks with dependency tracking
- `AgentTask` - Single task node with `agent_id`, `input`, and optional `parent_task_id`
- `fan_out` - Create parallel subtasks from a list of inputs
- `DepthLimiter` - Prevent unbounded recursive planning

## Planning Loop

```rust
use ancora_orchestrate::{TaskGraph, AgentTask, fan_out};

// Initial plan
let mut graph = TaskGraph::new();
let task = AgentTask::new("t1", "planner-agent", serde_json::json!("research climate"));
graph.add_task(task);

// Fan out sub-tasks for each step
let subtasks = fan_out("orch-1", "worker", vec![
    serde_json::json!("step-1"),
    serde_json::json!("step-2"),
], "t1");
for t in subtasks { graph.add_task(t); }
```

## Replanning

Replanning means creating new tasks after partial execution. The `ResultAggregator`
collects completed results; feed them back into a new round of `fan_out` calls to
replan based on evidence.

## Depth Control

Always wrap recursive planners:

```rust
let mut limiter = DepthLimiter::new(5);
limiter.enter()?; // errors if max depth exceeded
// ... plan ...
limiter.exit();
```

## Determinism

`TaskGraph` uses a `HashMap<task_id, task>`. Task insertion order is irrelevant;
execution order is determined by dependency resolution, which is deterministic given
the same input graph.
