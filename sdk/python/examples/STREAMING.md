# streaming

Streams token events from an agent run in real-time, printing each token
as it arrives rather than collecting all events first.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.streaming
```

## What it shows

- Using `run.stream_tokens()` to iterate over token text strings
- Flushing output incrementally for a live-typing feel
- Collecting the full response after the stream ends
