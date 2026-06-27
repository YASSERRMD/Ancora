# Testing Your Agents (TypeScript)

All Ancora TypeScript tests run offline by default with Vitest or Jest.

## Skip when native library is absent

```ts
import { describe, it, expect } from 'vitest'
import { Runtime, buildSpec } from 'ancora'

describe('SingleAgent', () => {
  it('runs a basic agent', async () => {
    let rt: Runtime
    try {
      rt = new Runtime()
    } catch {
      return // skip if native library not available
    }

    const spec = buildSpec({ model: 'llama3', instructions: 'Answer.' })
    const result = await rt.run(spec, 'What is 2+2?')
    expect(result.output).toBeTruthy()
  })
})
```

## Testing tool logic

Tool functions are plain TypeScript functions. Test them in isolation:

```ts
it('get_weather returns correct format', () => {
  const result = getWeather({ city: 'Cairo' })
  expect(result).toContain('Cairo')
  expect(result).toContain('22 C')
})
```

## Testing schema generation

```ts
import { z } from 'zod'

it('schema validates correctly', () => {
  const Schema = z.object({ headline: z.string(), body: z.string() })
  const parsed = Schema.parse({ headline: 'Test', body: 'Body' })
  expect(parsed.headline).toBe('Test')
})
```

## Testing with in-memory journal

```ts
import { MemoryStore, StoringTransport } from 'ancora'

it('journals a run', async () => {
  const store = new MemoryStore()
  const rt = new Runtime({ transport: new StoringTransport(store) })
  const spec = buildSpec({ model: 'llama3', instructions: 'Answer.' })
  await rt.run(spec, 'ping', { runId: 'test-run-1' })
  expect(store.hasRun('test-run-1')).toBe(true)
})
```

## Running tests

```bash
npx vitest run
# or
npx jest
```

## See also

- [Quickstart](quickstart.md)
- [Durability](durability.md)
