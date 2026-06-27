# streaming-chat

Demonstrates consuming agent events one at a time via `for await...of` on a
`RunHandle`, printing each token as it arrives rather than collecting all events.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/streaming-chat-example
```

## What it shows

- Iterating over events with `for await (const ev of handle)` 
- Filtering for `token` events and reading `ev.text`
- Handling large event streams (100+ tokens) without buffering all in memory
