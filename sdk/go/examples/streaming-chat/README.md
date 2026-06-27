# streaming-chat

Demonstrates consuming agent events in real-time via a Go channel returned
by `EventChan` rather than accumulating all events with `DrainEvents`.

## Run

```bash
cd sdk/go
go run ./examples/streaming-chat
```

## What it shows

- Creating an agent with `NewTransportAgent`
- Using `run.EventChan(ctx)` to receive events as they arrive
- Printing each event token as it streams in
- Applying a context deadline for safety
