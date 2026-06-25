# Migrating from CrewAI to Ancora

CrewAI organises work as `Crew` (a team of `Agent`s that each have a `role`,
`goal`, and `backstory`) executing `Task`s in sequence or in parallel.  Ancora
is graph-based and language-agnostic.

## Core model comparison

| CrewAI | Ancora | Notes |
|--------|--------|-------|
| `Agent(role, goal, backstory, tools)` | `Node { kind: Agent, spec: AgentSpec }` | The role/goal/backstory collapse into `instructions` |
| `Task(description, expected_output, agent)` | Graph node + edge | Each task becomes a node; dependencies become edges |
| `Crew(agents, tasks, process)` | `Graph` | Sequential process = linear edges; hierarchical = conditional edges |
| `crew.kickoff()` | `runtime.start_run(spec)` / `agent.run(spec)` | Returns an event stream, not a final string |
| `Tool(name, description, func)` | `LangchainTool` + `from_langchain()` | Direct adapter available |
| `Process.sequential` | Linear `Edge` chain | `A -> B -> C` |
| `Process.hierarchical` | `Edge { condition }` with a manager node | Manager node routes via conditional edges |

## Agent migration

### CrewAI agent definition

```python
researcher = Agent(
    role="Research Analyst",
    goal="Find accurate information on the topic",
    backstory="An expert researcher with a nose for facts.",
    tools=[search_tool, browse_tool],
    verbose=True,
)
```

### Ancora equivalent

```json
{
  "id": "researcher",
  "kind": "Agent",
  "spec": {
    "name": "researcher",
    "model_id": "claude-3-5-haiku-20241022",
    "instructions": "You are a Research Analyst. Your goal is to find accurate information on the topic. You have access to search and browse tools.",
    "tools": ["search", "browse"],
    "max_steps": 10
  }
}
```

## Task migration

CrewAI tasks map directly to graph nodes.  The `expected_output` becomes the
node's `output_schema_json`.

### CrewAI task

```python
research_task = Task(
    description="Research the market for electric vehicles.",
    expected_output="A bullet-point summary of the top 5 trends.",
    agent=researcher,
)
```

### Ancora graph node

```json
{
  "id": "research-ev-market",
  "kind": "Agent",
  "spec": {
    "name": "research-ev-market",
    "model_id": "claude-3-5-haiku-20241022",
    "instructions": "Research the market for electric vehicles. Produce a bullet-point summary of the top 5 trends.",
    "output_schema_json": "{\"type\":\"object\",\"required\":[\"trends\"],\"properties\":{\"trends\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}}}"
  }
}
```

## Process migration

### Sequential process

```python
crew = Crew(agents=[a, b, c], tasks=[t1, t2, t3], process=Process.sequential)
```

Ancora graph:

```json
{
  "entry_node": "t1",
  "nodes": [{"id":"t1",...}, {"id":"t2",...}, {"id":"t3",...}],
  "edges": [{"from":"t1","to":"t2"},{"from":"t2","to":"t3"}]
}
```

### Hierarchical process

Replace with a manager `Agent` node that reads sub-agent outputs and routes
via conditional edges.  The manager's instructions describe the delegation
logic that CrewAI's `manager_llm` handled automatically.

## Tool migration

```rust
// Existing CrewAI/LangChain tool -> Ancora via adapter
use ancora_tools::langchain_adapter::{LangchainTool, from_langchain};

let browse = LangchainTool::new("browse", "browses a URL", |url| {
    // existing browse logic
    Ok(format!("page content for: {}", url))
});
registry.register(from_langchain(browse));
```

## Output handling

CrewAI returns a string from `crew.kickoff()`.  Ancora returns a stream of
typed events.  Capture the final output from `RunEvent.Completed` or
`RunEvent.Token`:

```python
# Python SDK
for event in run.events():
    if event.kind == "token":
        print(event.text, end="", flush=True)
    elif event.kind == "completed":
        break
```

## Delegation and sub-agents

CrewAI's `allow_delegation=True` lets agents spawn sub-tasks.  In Ancora,
model sub-delegation is explicit: add child nodes and edges.  The parent
agent's instructions describe when to hand off to the child node.

## Step-by-step migration checklist

1. Map each `Agent` to an Ancora `AgentSpec` (collapse role/goal/backstory into `instructions`).
2. Map each `Task` to a graph node; set `output_schema_json` from `expected_output`.
3. Build edges that reflect the `Process` type (sequential = linear chain; hierarchical = conditional fan-out).
4. Wrap existing tools with `from_langchain()` or native SDK registration.
5. Replace `crew.kickoff()` with `runtime.start_run(spec)` and stream events.
6. Run the Ancora conformance suite to verify the migrated workflow.
