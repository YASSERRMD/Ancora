# Structured Output (TypeScript)

Force the agent to return a Zod-validated object instead of raw text.

## Define the schema

```ts
import { z } from 'zod'

const AnalysisResult = z.object({
  headline: z.string(),
  sentiment: z.enum(['positive', 'neutral', 'negative']),
  confidence: z.number().min(0).max(1),
})

type AnalysisResult = z.infer<typeof AnalysisResult>
```

## Pass the schema to buildSpec

```ts
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()

const spec = buildSpec({
  model: 'llama3',
  instructions: 'Analyse the sentiment of the user message.',
  outputSchema: AnalysisResult,
})

const result = await rt.run(spec, 'Ancora makes agent development simple!')
const analysis = result.parse(AnalysisResult)
console.log(analysis.headline, analysis.sentiment, analysis.confidence)
```

`result.parse(schema)` validates the raw JSON output against the Zod schema
and throws a `ZodError` if validation fails.

## Nested schemas

```ts
const Tag = z.object({ name: z.string(), score: z.number() })
const Report = z.object({
  summary: z.string(),
  tags: z.array(Tag),
})

const spec = buildSpec({
  model: 'llama3',
  instructions: 'Tag this text.',
  outputSchema: Report,
})

const report = (await rt.run(spec, '...')).parse(Report)
for (const tag of report.tags) {
  console.log(tag.name, tag.score)
}
```

## See also

- [Tools](tools.md)
- [API reference](api-reference.md)
