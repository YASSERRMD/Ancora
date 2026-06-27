# Migration from Mastra and Vercel AI SDK (TypeScript)

## From Vercel AI SDK

| Vercel AI SDK concept | Ancora equivalent |
|----------------------|-------------------|
| `generateText` | `rt.run(spec, prompt)` |
| `streamText` | `rt.stream(spec, prompt)` |
| `tool()` | `registry.register({ name, description, input, fn })` |
| `CoreMessage` | `AgentSpec.instructions` string |
| `LanguageModel` | `buildSpec.model` string + `ANCORA_MODEL_URL` |
| `experimental_generateObject` | `buildSpec({ outputSchema: zodSchema })` |

### Before (Vercel AI SDK)

```ts
import { generateText, tool } from 'ai'
import { openai } from '@ai-sdk/openai'
import { z } from 'zod'

const result = await generateText({
  model: openai('gpt-4o-mini'),
  tools: {
    get_weather: tool({
      description: 'Get the weather',
      parameters: z.object({ city: z.string() }),
      execute: async ({ city }) => `${city}: 22 C`,
    }),
  },
  prompt: 'What is the weather in Cairo?',
})
console.log(result.text)
```

### After (Ancora)

```ts
import { Runtime, buildSpec, ToolRegistry } from 'ancora'
import { z } from 'zod'

const registry = new ToolRegistry()
registry.register({
  name: 'get_weather',
  description: 'Get the weather',
  input: z.object({ city: z.string() }),
  fn: ({ city }) => `${city}: 22 C`,
})

const rt = new Runtime()
const spec = buildSpec({
  model: 'gpt-4o-mini',
  instructions: 'Answer weather questions.',
  tools: registry,
})

const result = await rt.run(spec, 'What is the weather in Cairo?')
console.log(result.output)
```

## From Mastra

| Mastra concept | Ancora equivalent |
|----------------|-------------------|
| `Agent` | `buildSpec + rt.run` |
| `Tool` | `registry.register(...)` |
| `Workflow` | `buildGraph + rt.runGraph` |
| `Step` | `GraphNode` in `buildGraph` |
| `MastraMemory` | `StoringTransport + SqliteStore` |

### Before (Mastra)

```ts
import { Agent, Tool } from '@mastra/core'

const agent = new Agent({
  name: 'weather-agent',
  instructions: 'Answer weather questions.',
  tools: { get_weather: weatherTool },
})

const result = await agent.generate('What is the weather in Cairo?')
console.log(result.text)
```

### After (Ancora)

```ts
const rt = new Runtime()
const spec = buildSpec({
  model: 'llama3',
  instructions: 'Answer weather questions.',
  tools: registry,
})
const result = await rt.run(spec, 'What is the weather in Cairo?')
console.log(result.output)
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Durability](durability.md)
