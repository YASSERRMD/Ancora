# event-chan

Demonstrates consuming Ancora agent events through a Go channel, suitable for
pipelines that process events as they arrive rather than waiting for all events.

## Run

```bash
go run ./examples/event-chan
```

## What it shows

- Starting a `TransportRun` and obtaining its event channel via `EventChan`
- Consuming events with a `for range` loop that exits when the channel closes
- The channel automatically closes when the event queue is empty
