# single_agent

Runs a single agent to completion and prints the kind of each event.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.single_agent
```

## What it shows

- Creating a `Runtime` and an `AgentSpec`
- Starting an agent run with `agent.run()`
- Streaming events with `run.stream_events()` and printing each one
