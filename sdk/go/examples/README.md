# Ancora Go SDK Examples

These examples demonstrate common patterns for the Ancora Go SDK.
All examples are runnable from the `sdk/go` directory.

## Prerequisites

```bash
# Build the Rust FFI staticlib first
make -C sdk/go build-ffi
```

## Examples

### single-agent

Runs a single agent to completion and prints its events.

```bash
go run ./examples/single-agent
```

Expected output:
```
started run: <uuid>
{"type":"started",...}
{"type":"completed",...}
done
```

### multi-agent-verifier

Starts two agents concurrently (main agent and verifier) and reports both runs.

```bash
go run ./examples/multi-agent-verifier
```

### human-in-loop

Starts a run, drains pre-resume events, reads a decision from stdin,
then resumes and drains post-resume events.

```bash
# Non-interactive: decision defaults to "approved"
go run ./examples/human-in-loop < /dev/null

# Interactive
go run ./examples/human-in-loop
```

### sqlite-persistence

Runs an agent with SQLite persistence; the run ID and events are stored in `example.db`.

```bash
go run ./examples/sqlite-persistence
```

### structured-output

Derives a JSON Schema from a Go struct with `SchemaFromStruct` and injects
the schema into the agent system prompt so the agent knows the expected output shape.

```bash
go run ./examples/structured-output
```

### streaming-chat

Consumes agent events in real-time via `EventChan` rather than waiting for all
events with `DrainEvents`.

```bash
go run ./examples/streaming-chat
```

### rag-lancedb

Builds an offline RAG context from a small document corpus and injects retrieved
passages into the agent system prompt before the run.

```bash
go run ./examples/rag-lancedb
```

### mcp-tool

Registers Go-native tool functions with `GoToolRegistry`, wires them into a
`RuntimeToolkit`, and invokes them by name with raw JSON input/output.

```bash
go run ./examples/mcp-tool
```

### glm-provider

Configures agent specs for the ChatGLM model family (glm-4, glm-4-flash,
glm-4-air, glm-3-turbo) and runs each variant through the standard transport.

```bash
go run ./examples/glm-provider
```

### durable-restart

Persists run events to a SQLite journal via `StoringTransport` and replays
them from the store after a simulated restart without re-running the agent.

```bash
go run ./examples/durable-restart
```

### cost-otel

Wraps an agent run in lightweight span tracking to record event counts,
duration, and estimated token cost -- mirroring OpenTelemetry span data.

```bash
go run ./examples/cost-otel
```
