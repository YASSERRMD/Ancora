# durable_restart

Persists run events to an in-process `RunJournal` backed by a `MemoryStore`
and replays them after a simulated restart, demonstrating the durable-restart
pattern without a live database.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.durable_restart
```

## What it shows

- Building a minimal `RunJournal` on top of `MemoryStore`
- Recording run IDs and appending events as they arrive
- Reading events back from the journal to simulate post-restart replay
- Tracking total run count across the session
