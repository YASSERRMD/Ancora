# Concurrency (TypeScript)

## Concurrent runs with `Promise.all`

`Runtime` is safe for concurrent use. Multiple `rt.run()` calls can overlap:

```ts
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()
const spec = buildSpec({ model: 'llama3', instructions: 'Summarise.' })

const prompts = ['Text A', 'Text B', 'Text C', 'Text D']

const results = await Promise.all(
  prompts.map(p => rt.run(spec, p))
)

for (const r of results) {
  console.log(r.output.slice(0, 60))
}
```

## Fan-out with `Promise.allSettled`

```ts
const settled = await Promise.allSettled(
  prompts.map(p => rt.run(spec, p))
)

for (const r of settled) {
  if (r.status === 'fulfilled') {
    console.log('OK:', r.value.output.slice(0, 40))
  } else {
    console.error('FAILED:', r.reason.message)
  }
}
```

## Limiting concurrency

Use a semaphore when running many agents against a resource-constrained
endpoint:

```ts
async function runWithLimit(items: string[], concurrency: number) {
  const results: string[] = []
  let index = 0

  async function worker() {
    while (index < items.length) {
      const i = index++
      const r = await rt.run(spec, items[i])
      results[i] = r.output
    }
  }

  await Promise.all(Array.from({ length: concurrency }, worker))
  return results
}

const outputs = await runWithLimit(prompts, 4)
```

## Streaming multiple agents concurrently

```ts
async function streamAll(prompts: string[]) {
  return Promise.all(
    prompts.map(async p => {
      const tokens: string[] = []
      for await (const ev of rt.stream(spec, p)) {
        if (ev.type === 'token') tokens.push(ev.token)
      }
      return tokens.join('')
    })
  )
}
```

## See also

- [Streaming](streaming.md)
- [Multi-agent graphs](multi-agent.md)
