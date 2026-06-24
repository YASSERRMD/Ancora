import { z } from 'zod'
import { Agent, buildSpec, defineTool, ToolRegistry, ToolBridge } from 'ancora'
import { makeOfflineRuntimeWithTool } from './offline-runtime'

const unreliableTool = defineTool({
  name: 'fetch_data',
  description: 'Fetch data from a source',
  schema: z.object({ url: z.string() }),
  handler: async ({ url }) => {
    if (url.includes('fail')) {
      throw new Error(`Network error fetching ${url}`)
    }
    return { data: `content from ${url}` }
  },
})

async function main() {
  const registry = new ToolRegistry().register(unreliableTool) as ToolRegistry
  const bridge = new ToolBridge(registry)

  const rt = makeOfflineRuntimeWithTool('fetch_data', { url: 'fail://bad-url' }, ['Handled', ' error.'])
  const agent = new Agent(rt)
  const handle = agent.run(buildSpec('claude', { tools: registry.specs }))

  for await (const ev of bridge.run(handle)) {
    if (ev.kind === 'tool_result') {
      const result = ev.result as Record<string, unknown>
      if ('error' in result) {
        console.log('Tool error was caught and handled:', result['error'])
      } else {
        console.log('Tool succeeded:', result)
      }
    } else if (ev.kind === 'token') {
      process.stdout.write(ev.text)
    } else if (ev.kind === 'completed') {
      process.stdout.write('\n')
    }
  }

  agent.free()
}

main().catch(console.error)
