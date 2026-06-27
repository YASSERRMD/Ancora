# Verifier and Consensus (TypeScript)

## Simple verifier

```ts
import { Runtime, buildSpec, buildGraph } from 'ancora'

const rt = new Runtime()

const graph = buildGraph({
  nodes: [
    { id: 'primary', model: 'llama3', instructions: 'Answer the user question.' },
    {
      id: 'verifier',
      model: 'llama3',
      instructions: "Verify the previous answer. Reply 'VERIFIED' or 'REJECTED: <reason>'.",
    },
  ],
  edges: [{ from: 'primary', to: 'verifier' }],
})

const result = await rt.runGraph(graph, 'What is the capital of Egypt?')
console.log(result.output)
```

## N-verifier consensus

```ts
const primarySpec = buildSpec({
  model: 'llama3',
  instructions: 'Answer the question.',
})

const verifierSpec = buildSpec({
  model: 'llama3',
  instructions: 'Is the following answer correct? Reply YES or NO.',
})

const candidate = (await rt.run(primarySpec, 'What is the capital of Egypt?')).output

const verdicts = await Promise.all(
  Array.from({ length: 3 }, () =>
    rt.run(verifierSpec, candidate).then(r => r.output.trim().toUpperCase().startsWith('YES'))
  )
)

const accepted = verdicts.filter(Boolean).length >= 2
console.log(accepted ? 'ACCEPTED' : 'REJECTED', candidate)
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Human-in-the-loop](human-in-the-loop.md)
