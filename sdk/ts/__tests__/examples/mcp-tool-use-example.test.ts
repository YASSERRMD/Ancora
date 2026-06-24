jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { z } from 'zod'
import { Agent, buildSpec, defineTool, ToolRegistry, ToolBridge, tokenText, collectEvents } from '../../index'
import { ToolBridgeEvent } from '../../tool-bridge'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

function makeToolRuntime(toolName: string, toolInput: Record<string, unknown>, replyTokens: string[]): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `mcp-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        { kind: 'tool_call', run_id: id, name: toolName, input: JSON.stringify(toolInput) },
      ])
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun(id: string): void {
      const q = runs.get(id) ?? []
      replyTokens.forEach(t => q.push({ kind: 'token', run_id: id, text: t }))
      q.push({ kind: 'completed', run_id: id })
      runs.set(id, q)
    },
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

const weatherTool = defineTool({
  name: 'get_weather',
  description: 'Get weather for a city',
  schema: z.object({ city: z.string() }),
  handler: async ({ city }) => ({ city, temp: '20C' }),
})

describe('mcp-tool-use example smoke test', () => {
  it('dispatches a tool_call and receives tool_result', async () => {
    const rt = makeToolRuntime('get_weather', { city: 'Paris' }, ['Weather:', ' 20C'])
    const registry = new ToolRegistry().register(weatherTool) as ToolRegistry
    const bridge = new ToolBridge(registry)
    const handle = new Agent(rt).run(buildSpec('test', { tools: registry.specs }))

    const events: ToolBridgeEvent[] = []
    for await (const ev of bridge.run(handle)) events.push(ev)
    const toolResult = events.find(e => e.kind === 'tool_result') as { kind: 'tool_result'; result: unknown } | undefined
    expect(toolResult).toBeDefined()
    expect((toolResult?.result as Record<string, unknown>)['city']).toBe('Paris')
  })

  it('tokenText includes reply tokens after tool call', async () => {
    const rt = makeToolRuntime('get_weather', { city: 'London' }, ['The', ' temp', ' is', ' 15C.'])
    const registry = new ToolRegistry().register(weatherTool) as ToolRegistry
    const bridge = new ToolBridge(registry)
    const handle = new Agent(rt).run(buildSpec('test', { tools: registry.specs }))

    const runEvents: RunEvent[] = []
    for await (const ev of bridge.run(handle)) {
      if (ev.kind !== 'tool_result') runEvents.push(ev)
    }
    expect(tokenText(runEvents)).toBe('The temp is 15C.')
  })

  it('tool_call is not yielded to caller (intercepted by bridge)', async () => {
    const rt = makeToolRuntime('get_weather', { city: 'Berlin' }, ['Done'])
    const registry = new ToolRegistry().register(weatherTool) as ToolRegistry
    const bridge = new ToolBridge(registry)
    const handle = new Agent(rt).run(buildSpec('test'))

    const kinds: string[] = []
    for await (const ev of bridge.run(handle)) {
      kinds.push(ev.kind)
    }
    expect(kinds).not.toContain('tool_call')
    expect(kinds).toContain('tool_result')
  })
})
