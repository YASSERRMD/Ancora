# Human-in-the-Loop (TypeScript)

Suspend a run at a tool call boundary and resume it with human input.

## Pattern

```ts
import { Runtime, buildSpec, ToolRegistry, RunStatus } from 'ancora'
import { z } from 'zod'
import * as readline from 'readline'

const rt = new Runtime()
const registry = new ToolRegistry()

registry.register({
  name: 'request_approval',
  description: 'Ask a human to approve an action.',
  input: z.object({ action: z.string() }),
  fn: ({ action }) => {
    throw new SuspendSignal(`Approve this action? ${action}`)
  },
})

const spec = buildSpec({
  model: 'llama3',
  instructions: 'Before modifying any file, call request_approval.',
  tools: registry,
})

const handle = rt.start(spec, 'Delete the temp directory.')
await handle.runUntilPause()

if (handle.status === RunStatus.PAUSED) {
  console.log('Approval required:', handle.pauseReason)
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout })
  const answer = await new Promise<string>(resolve => rl.question('Type YES to approve: ', resolve))
  rl.close()
  await handle.resume(answer)
}

const result = await handle.collect()
console.log(result.output)
```

## Resume with binary payload

```ts
await handle.resumeBytes(Buffer.from(JSON.stringify({ approved: true })))
```

## Timeout

```ts
const timer = setTimeout(() => handle.resume('TIMEOUT'), 30_000)
await handle.collect()
clearTimeout(timer)
```

## Async approval via webhook

```ts
// Store handle by run ID; resume when webhook fires
const handles = new Map<string, RunHandle>()

const handle = rt.start(spec, prompt)
handles.set(handle.runId, handle)
await handle.runUntilPause()

// Later, in a webhook handler:
app.post('/approve/:runId', async (req, res) => {
  const h = handles.get(req.params.runId)
  if (h) {
    await h.resume(req.body.decision)
    res.send('ok')
  }
})
```

## See also

- [Streaming](streaming.md)
- [Durability](durability.md)
