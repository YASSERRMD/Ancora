# mcp-tool

Demonstrates how to register Go-native tool functions with `GoToolRegistry`,
wire them into a `RuntimeToolkit`, and invoke them directly -- the same
mechanism used when an agent calls an MCP tool during a live run.

## Run

```bash
cd sdk/go
go run ./examples/mcp-tool
```

## What it shows

- Creating a `RuntimeToolkit` from a `Runtime`
- Registering multiple `ToolFunc` implementations
- Invoking tools by name with raw JSON input/output
- Checking tool registration with `Has` and `Count`
