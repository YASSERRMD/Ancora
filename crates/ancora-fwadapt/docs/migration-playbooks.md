# Migration Playbooks

Step-by-step guides for migrating from each supported framework to Ancora.

## From LangChain

1. Collect your `Tool` definitions (name + description + callable).
2. Call `import_langchain_tools(defs)` to get `AncoraToolAdapter` instances.
3. Register the adapters in the Ancora tool registry.
4. Replace Python `tool.run()` calls with `adapter.run()`.

The `ancora_to_langchain` module allows a phased migration: keep LangChain
as the orchestrator while individual tools are re-implemented in Ancora.

## From LangGraph

1. Export your graph as a list of nodes and edges.
2. Call `map_langgraph_to_stages(&graph)` to get an ordered `Vec<AncoraStage>`.
3. Map each stage to an Ancora pipeline step.

LangGraph cycles are rejected at mapping time.

## From CrewAI

1. Describe your agents (name, role, goal, backstory) as `CrewAIAgent` structs.
2. Describe tasks as `CrewAITask` with `assigned_to` matching an agent name.
3. Call `map_crewai_to_ancora(definition)` to get an `AncoraCrewPlan`.

## From MCP

1. Register each MCP tool definition with `McpToolRegistry::register()`.
2. Use `registry.validate_call()` to guard execution before dispatching.

## From OpenAI Agents SDK

1. Create a `HandoffBridge`.
2. Register each Ancora agent function with `bridge.register_agent(name, fn)`.
3. When an OpenAI handoff arrives, call `bridge.execute_handoff(&handoff)`.

## From Semantic Kernel

1. Translate your SK plugin YAML/JSON into `SKPluginDef`.
2. Call `import_sk_plugin(plugin)` to get `Vec<AncoraSkToolSpec>`.
3. Register the specs in the Ancora tool registry.
