# MCP and A2A (TypeScript)

## Using MCP tools

```ts
const spec = buildSpec({
  model: 'llama3',
  instructions: 'Use the weather tool.',
  mcpServers: ['http://localhost:8090/mcp'],
})
```

## Exposing tools via MCP

```ts
import { McpServer, ToolRegistry } from 'ancora'

const registry = new ToolRegistry()
registry.register({ name: 'get_weather', description: '...', input: z.object({ city: z.string() }), fn: ({ city }) => `${city}: 22 C` })

const server = new McpServer(registry, { port: 8090 })
server.listenInBackground()
```

## A2A: accepting external tasks

```ts
import { A2AServer } from 'ancora'

const a2a = new A2AServer(rt, { port: 8091 })
a2a.listenInBackground()
```

## A2A: delegating to an external agent

```ts
import { A2AClient } from 'ancora'

const delegate = new A2AClient('http://external-agent:8091')
const result = await delegate.delegate({
  instructions: 'Summarise this document.',
  context: docText,
})
console.log(result.output)
```

## See also

- [MCP and A2A concept](../../concepts/mcp-and-a2a.md)
- [Tools](tools.md)
