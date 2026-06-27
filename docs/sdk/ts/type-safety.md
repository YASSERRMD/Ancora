# Type Safety (TypeScript)

Ancora's TypeScript SDK is written in TypeScript and ships declaration files.
All public APIs are fully typed with no `any` leakage in normal usage.

## Strict mode

Enable strict TypeScript for the best experience:

```json
{
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true
  }
}
```

## Typed tool inputs with Zod

Tool input types flow through automatically from the Zod schema:

```ts
import { z } from 'zod'
import { ToolRegistry } from 'ancora'

const registry = new ToolRegistry()

registry.register({
  name: 'get_price',
  description: 'Look up the price of an item.',
  input: z.object({
    sku: z.string(),
    currency: z.enum(['USD', 'EUR']).default('USD'),
  }),
  fn: ({ sku, currency }) => {
    // sku: string, currency: 'USD' | 'EUR' -- fully inferred
    return `${sku}: 29.99 ${currency}`
  },
})
```

## Typed structured output

```ts
import { z } from 'zod'
import { buildSpec, Runtime } from 'ancora'

const Summary = z.object({ headline: z.string(), body: z.string() })
type Summary = z.infer<typeof Summary>

const spec = buildSpec({
  model: 'llama3',
  instructions: 'Summarise.',
  outputSchema: Summary,
})

const rt = new Runtime()
const result = await rt.run(spec, 'Ancora is a durable runtime.')
const summary: Summary = result.parse(Summary)   // typed Summary, no `any`
console.log(summary.headline)                     // string
```

## Generic RunEvent narrowing

```ts
for await (const event of rt.stream(spec, prompt)) {
  if (event.type === 'completed') {
    // TypeScript narrows event to { type: 'completed'; output: string; usage: TokenUsage }
    console.log(event.output)
  }
}
```

## PolicySpec type

```ts
import type { PolicySpec } from 'ancora'

const policy = {
  allowRegions: ['us-east-1'],
  maxWriteTools: 2,
} satisfies PolicySpec   // compile-time check, no runtime overhead
```

## See also

- [Structured output](structured-output.md)
- [API reference](api-reference.md)
