# Defining Tools with Zod (TypeScript)

Tools are typed functions registered with a `ToolRegistry` and passed to
`buildSpec`. The agent calls them during a run; return values are injected
back into the model context.

## Register a tool

```ts
import { Runtime, buildSpec, ToolRegistry } from 'ancora'
import { z } from 'zod'

const registry = new ToolRegistry()

registry.register({
  name: 'get_weather',
  description: 'Return the current weather for a city.',
  input: z.object({ city: z.string() }),
  fn: ({ city }) => `${city}: 22 C, sunny`,
})

const spec = buildSpec({
  model: 'llama3',
  instructions: 'Use get_weather to answer weather questions.',
  tools: registry,
})

const rt = new Runtime()
const result = await rt.run(spec, 'What is the weather in Cairo?')
console.log(result.output)
```

## Zod schema validation

Ancora validates tool inputs against the Zod schema before calling your
function. If the model sends a malformed input, the tool call fails gracefully
and the agent retries.

```ts
registry.register({
  name: 'get_price',
  description: 'Look up the price of a product by SKU.',
  input: z.object({
    sku: z.string().regex(/^[A-Z]{3}-\d{4}$/),
    currency: z.enum(['USD', 'EUR', 'GBP']).default('USD'),
  }),
  fn: ({ sku, currency }) => `${sku}: 29.99 ${currency}`,
})
```

## Async tools

```ts
registry.register({
  name: 'fetch_url',
  description: 'Fetch the text content of a URL.',
  input: z.object({ url: z.string().url() }),
  fn: async ({ url }) => {
    const resp = await fetch(url)
    const text = await resp.text()
    return text.slice(0, 500)
  },
})
```

## Tool effect classes

```ts
import { EffectClass } from 'ancora'

registry.register({
  name: 'write_file',
  description: 'Write content to a file.',
  input: z.object({ path: z.string(), content: z.string() }),
  effect: EffectClass.WRITE,
  fn: async ({ path, content }) => {
    await fs.promises.writeFile(path, content)
    return 'ok'
  },
})
```

## See also

- [Structured output](structured-output.md)
- [Policy](policy.md)
