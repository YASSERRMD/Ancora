# human-in-loop

Demonstrates pausing an agent run, collecting pre-resume events, calling
`handle.resume()` with a decision, and collecting post-resume events.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/human-in-loop-example
```

## What it shows

- Collecting pre-resume events via async iteration
- Calling `handle.resume(decision)` with a string or Uint8Array
- Collecting post-resume events with `handle.run(decision)`
