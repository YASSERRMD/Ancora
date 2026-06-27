# single-agent

Demonstrates starting a single agent run with `buildSpec`, collecting events
with `collectEvents`, and reading the full token text with `tokenText`.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/single-agent-example
```

## What it shows

- Building an agent spec with `buildSpec(model, { maxTokens })`
- Starting a run via `agent.run(spec)` returning a `RunHandle`
- Collecting all events with `collectEvents` and reading tokens with `tokenText`
