# mcp_tool_use

Registers Go-native tool functions (web search, weather, email, calculator)
with a `ToolRegistry`, dispatches them by name, and wires them into an agent
spec -- the same mechanism used when an agent calls an MCP tool.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.mcp_tool_use
```

## What it shows

- Registering tools with `EffectClass` annotations (PURE, READ, WRITE)
- Dispatching tool calls directly via `registry.dispatch()`
- Wiring the registry into an `AgentSpec` and running the agent
