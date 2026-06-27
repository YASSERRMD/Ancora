# multi_agent

Starts a primary agent and a verifier agent concurrently, demonstrating how
to track multiple independent runs in parallel.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.multi_agent
```

## What it shows

- Launching two agents concurrently with `asyncio.gather`
- Collecting events from each run independently
- Reporting per-run event counts and IDs
