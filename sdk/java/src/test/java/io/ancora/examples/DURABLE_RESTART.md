# Durable Restart Example

Demonstrates persisting run events to an in-memory `RunJournal` and
replaying them after a simulated process restart without re-executing the
agent.

## What it tests

- `RunJournal.recordRun` is idempotent (calling twice keeps `runCount()` at 1)
- `appendEvent` and `eventsForRun` round-trip correctly
- Multiple runs are tracked independently
- Live events streamed via `handle.events()` can be written to the journal

## Pattern

```java
RunJournal journal = new RunJournal();

try (Agent agent = new Agent()) {
    AgentSpec spec = new AgentSpec("local-model", "Persist my events.", null, null, null);
    var handle = agent.run(spec);
    String runId = handle.runId();
    journal.recordRun(runId);

    for (RunEvent ev : handle.events())
        journal.appendEvent(runId, ev.toString());

    // Simulate restart: replay from journal
    List<String> replayed = journal.eventsForRun(runId);
    assertFalse(replayed.isEmpty());
}
```

## Offline behaviour

`RunJournal` is entirely in-process. The live-events test uses
`Assumptions.assumeTrue(AncoraNative.AVAILABLE)`.
