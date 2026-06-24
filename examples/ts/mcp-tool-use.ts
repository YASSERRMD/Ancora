import { z } from 'zod'
import { Agent, buildSpec, defineTool, ToolRegistry, ToolBridge, tokenText, collectEvents } from 'ancora'
import { makeOfflineRuntimeWithTool } from './offline-runtime'

const weatherTool = defineTool({
  name: 'get_weather',
  description: 'Get the current weather for a given city.',
  schema: z.object({
    city: z.string().describe('The city name'),
  }),
  handler: async ({ city }) => {
    return { city, temperature: '18C', condition: 'cloudy' }
  },
})

const searchTool = defineTool({
  name: 'web_search',
  description: 'Search the web for information.',
  schema: z.object({
    query: z.string().describe('The search query'),
  }),
  handler: async ({ query }) => {
    return { results: [`Result 1 for "${query}"`, `Result 2 for "${query}"`] }
  },
})

async function main() {
  const registry = new ToolRegistry()
  registry.register(weatherTool).register(searchTool)

  const spec = buildSpec('claude-3-5-sonnet', {
    instructions: 'Use tools to answer questions.',
    tools: registry.specs,
    maxTokens: 1024,
  })

  const rt = makeOfflineRuntimeWithTool('get_weather', { city: 'Paris' }, ['The', ' weather', ' is', ' 18C', '.'])
  const agent = new Agent(rt)
  const bridge = new ToolBridge(registry)
  const handle = agent.run(spec)

  console.log('Running MCP tool use example...')

  const events = await collectEvents(bridge.run(handle))

  for (const ev of events) {
    if (ev.kind === 'tool_call') {
      console.log(`Tool called: ${ev.name}`)
    } else if (ev.kind === 'tool_result') {
      console.log(`Tool result:`, JSON.stringify(ev.result))
    }
  }

  console.log('Final response:', tokenText(events))
  agent.free()
}

main().catch(console.error)
