# Quickstart (TypeScript)

Run a minimal agent locally in under five minutes.

## Prerequisites

- `npm install ancora`
- Ollama running: `ollama serve && ollama pull llama3`

## Minimal example

```ts
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()

const spec = buildSpec({
  model: 'llama3',
  instructions: 'Answer concisely.',
})

const result = await rt.run(spec, 'What is a durable agent?')
console.log(result.output)
```

## Important: use `instructions`, not `systemPrompt`

```ts
// CORRECT
buildSpec({ model: 'llama3', instructions: 'Answer.' })

// WRONG -- will not compile
// buildSpec({ model: 'llama3', systemPrompt: 'Answer.' })
```

## Run it

```bash
npx ts-node quickstart.ts
# or
node --loader ts-node/esm quickstart.ts
```

## What happened

1. `new Runtime()` initialises the native Ancora engine via N-API.
2. `buildSpec(...)` declares the model and instructions.
3. `rt.run(spec, prompt)` submits the prompt and awaits completion.

## Next steps

- [Defining tools](tools.md) -- give the agent TypeScript functions
- [Structured output](structured-output.md) -- parse output with Zod
- [Streaming](streaming.md) -- consume tokens as an AsyncIterable
