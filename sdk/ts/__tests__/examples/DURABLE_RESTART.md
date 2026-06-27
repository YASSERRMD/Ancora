# durable-restart

Demonstrates persisting run events to an in-process `RunJournal` and replaying
them after a simulated restart without re-running the agent.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/durable-restart-example
```

## What it shows

- A `RunJournal` class backed by a `Map` (mirrors SQLite or Redis in production)
- Recording run IDs and appending events as they arrive
- Reading events back for replay after a simulated crash
- Tracking total run count across the session
