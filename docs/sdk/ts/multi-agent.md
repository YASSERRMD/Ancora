# Multi-Agent Graphs (TypeScript)

Orchestrate multiple agents by defining a `GraphSpec` with nodes and edges.

## Define a graph

```ts
import { Runtime, buildGraph } from 'ancora'

const rt = new Runtime()

const graph = buildGraph({
  nodes: [
    { id: 'writer', model: 'llama3', instructions: 'Write a paragraph on the given topic.' },
    { id: 'reviewer', model: 'llama3', instructions: 'Review the paragraph and suggest one improvement.' },
  ],
  edges: [
    { from: 'writer', to: 'reviewer' },
  ],
})

const result = await rt.runGraph(graph, 'durable AI agents')
console.log(result.output)
```

## Fan-out graph

```ts
const graph = buildGraph({
  nodes: [
    { id: 'writer', model: 'llama3', instructions: 'Write a paragraph.' },
    { id: 'reviewer_a', model: 'llama3', instructions: 'Focus on clarity.' },
    { id: 'reviewer_b', model: 'llama3', instructions: 'Focus on accuracy.' },
  ],
  edges: [
    { from: 'writer', to: 'reviewer_a' },
    { from: 'writer', to: 'reviewer_b' },
  ],
})
```

`reviewer_a` and `reviewer_b` run concurrently once `writer` completes.

## Accessing node outputs

```ts
const run = rt.startGraph(graph, 'durable AI agents')
const outputs = await run.nodeOutputs()
for (const [nodeId, output] of Object.entries(outputs)) {
  console.log(nodeId, '->', output.slice(0, 80))
}
```

## See also

- [Verifier](verifier.md)
- [Orchestration graph concept](../../concepts/orchestration-graph.md)
