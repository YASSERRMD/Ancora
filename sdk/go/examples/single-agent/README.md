# single-agent

Runs a single Ancora agent to completion and prints the event log.

## Run

```bash
go run ./examples/single-agent
```

## Expected output

```
started run: <uuid>
{"type":"started",...}
{"type":"completed",...}
done
```

## What it shows

- Creating a `Runtime` and freeing it with `defer`
- Building an `AgentSpec` via `NewAgentSpec`
- Wrapping the runtime in a `CgoTransport`
- Starting a run with `TransportAgent.Start`
- Draining all events with `TransportRun.DrainEvents`
