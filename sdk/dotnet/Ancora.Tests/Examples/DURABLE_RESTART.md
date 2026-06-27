# Durable Restart Example

Demonstrates persisting run events to an in-memory `RunJournal` and
replaying them after a simulated process restart, without re-executing the
agent.

## What it tests

- `RunJournal.RecordRun` is idempotent (calling twice keeps `RunCount == 1`)
- `AppendEvent` and `EventsForRun` round-trip correctly
- Multiple runs are tracked independently
- Live events streamed via `EventsAsync()` can be written to the journal

## Pattern

```csharp
var journal = new RunJournal();

using var agent = new Agent();
var handle = agent.Run(new AgentSpec("local-model", "Persist my events."));
var runId = handle.RunId;
journal.RecordRun(runId);

await foreach (var ev in handle.EventsAsync())
    journal.AppendEvent(runId, ev.Kind);

// Simulate restart: replay from journal
var replayed = journal.EventsForRun(runId);
Assert.NotEmpty(replayed);
```

## Offline behaviour

`RunJournal` is entirely in-process. The live-events test catches
`DllNotFoundException`.
