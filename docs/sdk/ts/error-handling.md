# Error Handling (TypeScript)

## Exception types

```ts
import {
  AncorError,
  NativeError,
  RunFailedError,
  PolicyViolationError,
  TimeoutError,
  JournalError,
} from 'ancora'
```

All Ancora exceptions extend `AncorError`.

## Catching run errors

```ts
import { RunFailedError } from 'ancora'

try {
  const result = await rt.run(spec, 'What is 2+2?')
  console.log(result.output)
} catch (e) {
  if (e instanceof RunFailedError) {
    console.error('Run failed:', e.message, 'run_id:', e.runId)
  } else {
    throw e
  }
}
```

## Retry on transient errors

```ts
async function runWithRetry(
  rt: Runtime,
  spec: AgentSpec,
  prompt: string,
  maxAttempts = 3
): Promise<RunResult> {
  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    try {
      return await rt.run(spec, prompt)
    } catch (e) {
      if (e instanceof RunFailedError && e.isTransient && attempt < maxAttempts - 1) {
        await new Promise(r => setTimeout(r, 2 ** attempt * 1000))
        continue
      }
      throw e
    }
  }
  throw new Error('unreachable')
}
```

## Streaming error handling

```ts
try {
  for await (const event of rt.stream(spec, prompt)) {
    if (event.type === 'error') {
      console.error('Stream error:', event.message)
      break
    }
    if (event.type === 'token') process.stdout.write(event.token)
  }
} catch (e) {
  console.error('Unexpected stream error:', e)
}
```

## Zod parse errors

```ts
import { z } from 'zod'

const Schema = z.object({ headline: z.string() })

try {
  const data = result.parse(Schema)
  console.log(data.headline)
} catch (e) {
  if (e instanceof z.ZodError) {
    console.error('Parse error:', e.errors)
  }
}
```

## See also

- [Troubleshooting](troubleshooting.md)
- [Durability](durability.md)
