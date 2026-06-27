# Memory and RAG (TypeScript)

## Offline keyword retrieval

```ts
interface Passage { key: string; content: string }

function keywordRetrieve(corpus: Passage[], query: string, topK = 3): Passage[] {
  const terms = new Set(query.toLowerCase().match(/\w+/g) ?? [])
  const scored = corpus.map(p => {
    const pTerms = new Set((p.content.toLowerCase().match(/\w+/g) ?? []))
    const overlap = [...terms].filter(t => pTerms.has(t)).length
    return { score: overlap, passage: p }
  })
  return scored
    .filter(s => s.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, topK)
    .map(s => s.passage)
}

const corpus: Passage[] = [
  { key: 'doc1', content: 'Ancora provides durable agent orchestration.' },
  { key: 'doc2', content: 'LanceDB is an embedded vector database.' },
  { key: 'doc3', content: 'Zod validates structured data in TypeScript.' },
]

const hits = keywordRetrieve(corpus, 'durable agent')
const context = hits.map(h => h.content).join('\n')
```

## Inject context into the agent

```ts
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()

const spec = buildSpec({
  model: 'llama3',
  instructions: `Answer using only the following context:\n\n${context}`,
})

const result = await rt.run(spec, 'What does Ancora provide?')
console.log(result.output)
```

## Context as a tool

```ts
import { z } from 'zod'

registry.register({
  name: 'retrieve',
  description: 'Retrieve relevant passages for a query.',
  input: z.object({ query: z.string() }),
  fn: ({ query }) => keywordRetrieve(corpus, query).map(h => h.content).join('\n'),
})
```

## LanceDB vector retrieval (Node.js)

```ts
import { connect } from 'vectordb'

const db = await connect('/var/lib/myapp/vectors')
const table = await db.openTable('passages')
const results = await table.search(embed('durable agent')).limit(3).execute()
const context = results.map(r => r.content as string).join('\n')
```

## See also

- [Vector stores](vector-stores.md)
- [Providers](providers.md)
