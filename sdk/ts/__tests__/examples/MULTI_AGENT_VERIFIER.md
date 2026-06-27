# multi-agent-verifier

Demonstrates running a primary agent and a verifier agent concurrently using
`Promise.all`, collecting events from each independently.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/multi-agent-verifier-example
```

## What it shows

- Starting two agents concurrently with `Promise.all`
- Verifying both produce `started` and `completed` events
- Confirming each run has a distinct run ID
