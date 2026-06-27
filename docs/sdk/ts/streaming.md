# Streaming (TypeScript)

Consume run events as an `AsyncIterable` instead of waiting for the full
response.

## Basic streaming

```ts
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()
const spec = buildSpec({ model: 'llama3', instructions: 'Tell a short story.' })

for await (const event of rt.stream(spec, 'Once upon a time...')) {
  if (event.type === 'token') {
    process.stdout.write(event.token)
  }
}
console.log()
```

## Accumulating tokens

```ts
const tokens: string[] = []

for await (const event of rt.stream(spec, prompt)) {
  if (event.type === 'token') {
    tokens.push(event.token)
  }
}

const fullText = tokens.join('')
```

## Event types

| `event.type` | Fields | Description |
|-------------|--------|-------------|
| `'started'` | `runId` | Run has begun |
| `'token'` | `token` | One model output token |
| `'tool_call'` | `name`, `input` | Agent called a tool |
| `'completed'` | `output`, `usage` | Run finished |
| `'resumed'` | `runId` | Run resumed after pause |
| `'error'` | `message` | Run failed |

## Streaming into a WebSocket

```ts
import { WebSocket } from 'ws'

async function streamToWs(ws: WebSocket, spec: AgentSpec, prompt: string) {
  for await (const event of rt.stream(spec, prompt)) {
    if (event.type === 'token') {
      ws.send(JSON.stringify({ type: 'token', token: event.token }))
    } else if (event.type === 'completed') {
      ws.send(JSON.stringify({ type: 'done' }))
    }
  }
}
```

## Streaming with AbortController

```ts
const controller = new AbortController()
setTimeout(() => controller.abort(), 10_000)   // 10 second timeout

try {
  for await (const event of rt.stream(spec, prompt, { signal: controller.signal })) {
    if (event.type === 'token') process.stdout.write(event.token)
  }
} catch (e) {
  if ((e as Error).name === 'AbortError') console.log('\nAborted.')
}
```

## See also

- [Human-in-the-loop](human-in-the-loop.md)
- [Quickstart](quickstart.md)
