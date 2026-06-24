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
