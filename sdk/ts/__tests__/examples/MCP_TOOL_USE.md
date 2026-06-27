# mcp-tool-use

Demonstrates registering Go-native tools with `defineTool` and `ToolRegistry`,
wiring them into an agent run, and dispatching tool calls via `ToolBridge`.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/mcp-tool-use-example
```

## What it shows

- Defining tools with `defineTool` using Zod input schemas
- Registering tools in a `ToolRegistry`
- Wiring a `ToolBridge` to dispatch tool_call events during the run
