import { z } from 'zod'
import { Agent, buildSpec, defineTool, createToolBridge, tokenText } from 'ancora'
import { makeOfflineRuntimeWithTool } from './offline-runtime'

const convertTool = defineTool({
  name: 'convert_currency',
  description: 'Convert an amount from one currency to another',
  schema: z.object({
    amount: z.number(),
    from: z.string(),
    to: z.string(),
  }),
  handler: async ({ amount, from, to }) => {
    const rate = from === 'USD' && to === 'EUR' ? 0.92 : 1.0
    return { result: amount * rate, from, to }
  },
})

const formatTool = defineTool({
  name: 'format_number',
  description: 'Format a number for display',
  schema: z.object({ value: z.number(), currency: z.string() }),
  handler: async ({ value, currency }) => `${currency} ${value.toFixed(2)}`,
})

async function main() {
  const bridge = createToolBridge(convertTool, formatTool)
  const rt = makeOfflineRuntimeWithTool('convert_currency', { amount: 100, from: 'USD', to: 'EUR' }, ['Converted', ' successfully.'])
  const agent = new Agent(rt)
  const handle = agent.run(buildSpec('claude', { tools: bridge.registry.specs }))

  const events = []
  for await (const ev of bridge.run(handle)) {
    if (ev.kind === 'tool_result') {
      console.log('Tool result:', JSON.stringify(ev.result))
    }
    events.push(ev)
  }

  agent.free()
}

main().catch(console.error)
