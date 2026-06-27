# Durability and Restart Recovery (TypeScript)

## Enable durability

```ts
import { Runtime, SqliteStore, StoringTransport } from 'ancora'

const store = new SqliteStore('/var/lib/myapp/journal.db')
const rt = new Runtime({ transport: new StoringTransport(store) })
```

With a `StoringTransport`, every run is journalled automatically. If the
process restarts mid-run, replay the journal to continue from the last
checkpoint:

```ts
const handle = rt.resume('run-abc-123')
const result = await handle.collect()
console.log(result.output)
```

## Deterministic run IDs

```ts
const result = await rt.run(spec, 'Summarise the report.', {
  runId: 'report-summary-2026-06-28',
})
```

Re-running with the same `runId` replays completed activities and re-runs
only the remaining steps.

## In-memory store (tests)

```ts
import { MemoryStore, StoringTransport } from 'ancora'

const rt = new Runtime({ transport: new StoringTransport(new MemoryStore()) })
```

## Idempotency key templates

```ts
import { ToolSpec } from 'ancora'
import { z } from 'zod'

const sendEmailSpec: ToolSpec = {
  name: 'send_email',
  description: 'Send an email.',
  input: z.object({ to: z.string(), subject: z.string(), body: z.string() }),
  idempotencyKeyTemplate: 'send_email/{runId}/{seq}',
  fn: async ({ to, subject, body }) => {
    // send email
    return 'sent'
  },
}
```

## See also

- [Observability](observability.md)
- [Durability concept](../../concepts/durability-and-replay.md)
