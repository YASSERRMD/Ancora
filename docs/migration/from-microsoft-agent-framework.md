# Migrating from Microsoft Agent Framework (Semantic Kernel / AutoGen) to Ancora

Microsoft's agent landscape includes Semantic Kernel (SK) and AutoGen.  This
guide covers both, with a focus on the concepts that map cleanly to Ancora.

## Semantic Kernel

### Core model comparison

| Semantic Kernel | Ancora | Notes |
|-----------------|--------|-------|
| `Kernel` | `Runtime` | Top-level execution context |
| `KernelPlugin` / `KernelFunction` | `Tool` | Register via `ToolRegistry` |
| `IChatCompletionService` | Model adapter (configured in `AgentSpec.model_id`) | |
| `ChatHistory` | Journal + node output | Ancora journals replace mutable history |
| `Planner` (`SequentialPlanner`, `StepwisePlanner`) | `Graph` with edges | Graph replaces the planner |
| `FunctionCallingStepwisePlanner` | `Agent` node with tools | Multi-step tool use is built in |
| `KernelArguments` | `AgentSpec.instructions` (parametrised) | |

### Plugin migration

Semantic Kernel plugins expose C#/Python methods decorated with
`[KernelFunction]` or `@kernel_function`.  Map these to Ancora tools:

```csharp
// Semantic Kernel plugin
public class MathPlugin
{
    [KernelFunction("add")]
    [Description("Add two numbers")]
    public double Add(double a, double b) => a + b;
}
```

Ancora equivalent (Rust adapter or Java `@Tool`):

```java
// Java SDK
public class MathTools {
    @Tool(description = "Add two numbers")
    public String add(
        @ToolInput(description = "first number") double a,
        @ToolInput(description = "second number") double b) {
        return String.valueOf(a + b);
    }
}
List<ToolRegistration> tools = ToolRegistry.registerAll(runtime, new MathTools());
```

```rust
// Rust adapter
use ancora_tools::langchain_adapter::{LangchainTool, from_langchain};

let add = LangchainTool::new("add", "adds two numbers", |input| {
    let v: serde_json::Value = serde_json::from_str(input).map_err(|e| e.to_string())?;
    let a = v["a"].as_f64().unwrap_or(0.0);
    let b = v["b"].as_f64().unwrap_or(0.0);
    Ok((a + b).to_string())
});
registry.register(from_langchain(add));
```

### Planner migration

Semantic Kernel planners take a goal and produce a plan (sequence of function
calls).  In Ancora, define the plan explicitly as a graph:

```csharp
// SK: SequentialPlanner produces a plan for "research and summarise"
var plan = await planner.CreatePlanAsync(kernel, "Research EV market and summarise.");
await plan.InvokeAsync(kernel);
```

Ancora: build the graph manually (or generate it from a meta-agent):

```json
{
  "entry_node": "researcher",
  "nodes": [
    { "id": "researcher", "spec": { "instructions": "Research EV market.", ... } },
    { "id": "summariser", "spec": { "instructions": "Summarise the research output.", ... } }
  ],
  "edges": [{ "from": "researcher", "to": "summariser" }]
}
```

## AutoGen

### Core model comparison

| AutoGen | Ancora | Notes |
|---------|--------|-------|
| `AssistantAgent` | `Agent` node | |
| `UserProxyAgent` | `HumanNode` | Human-in-the-loop via `runtime.suspend()` |
| `GroupChat` | `Graph` with multiple agent nodes | |
| `GroupChatManager` | Manager `Agent` node with conditional edges | |
| `ConversableAgent.initiate_chat` | `runtime.start_run(spec)` + event stream | |
| Function calling / tool use | `ToolSpec` / tool registration | |
| Memory (`TextMemory`) | External tool + Ancora journal | |

### Multi-agent chat migration

AutoGen's `GroupChat` has agents taking turns.  In Ancora, model turn-taking
as a linear or fan-out graph:

```python
# AutoGen group chat
assistant = AssistantAgent("assistant", llm_config={...})
critic    = AssistantAgent("critic", llm_config={...})
manager   = GroupChatManager(groupchat=GroupChat([assistant, critic], messages=[], max_round=5))
user.initiate_chat(manager, message="Review this code.")
```

Ancora equivalent:

```json
{
  "entry_node": "assistant",
  "nodes": [
    { "id": "assistant", "spec": { "instructions": "Draft a response.", ... } },
    { "id": "critic",    "spec": { "instructions": "Critique the assistant's response.", ... } }
  ],
  "edges": [{ "from": "assistant", "to": "critic" }]
}
```

For multi-round conversation, use a loop edge with a termination condition:

```json
{
  "edges": [
    { "from": "critic", "to": "assistant", "condition": "$.round < 5" },
    { "from": "critic", "to": "__end__",   "condition": "$.round >= 5" }
  ]
}
```

### Human-in-the-loop migration

AutoGen's `UserProxyAgent` proxies human input.  Ancora uses `HumanNode` or
`runtime.suspend(run_id, prompt)` / `runtime.resume(run_id, decision)`:

```python
# Python SDK
run = runtime.start_run(spec)
for event in run.events():
    if event.kind == "human_decision_requested":
        decision = input(event.prompt)
        run.resume(decision)
```

## Step-by-step migration checklist

### Semantic Kernel

1. Map each `KernelPlugin` to a `ToolRegistry` registration (via language SDK
   or `from_langchain()` adapter).
2. Replace `Planner` logic with an explicit `Graph` (or use a meta-agent that
   produces graph JSON).
3. Replace `ChatHistory` mutations with journal event reads (available via
   `JournalStore.read(run_id)`).
4. Replace `kernel.InvokeAsync()` with `runtime.start_run(spec)` and stream
   events.

### AutoGen

1. Map each `ConversableAgent` to an `AgentNode`.
2. Map `GroupChatManager` turn-taking to a graph with edges and optional
   round-count conditions.
3. Replace `UserProxyAgent` with `HumanNode` or `runtime.suspend()`.
4. Replace `initiate_chat()` with `runtime.start_run(spec)` and handle the
   `human_decision_requested` event in the event loop.
5. Run the Ancora conformance suite to verify the migrated workflow.
