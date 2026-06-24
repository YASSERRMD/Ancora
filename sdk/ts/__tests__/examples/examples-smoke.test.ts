jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { validateSpec, buildSpec } from '../../wire'
import { z } from 'zod'
import { defineTool, ToolRegistry } from '../../tools'
import { ToolBridge, createToolBridge } from '../../tool-bridge'
import { Agent } from '../../agent'
import { Runtime } from '../../index'
import { tokenText } from '../../helpers'
import { RunEvent } from '../../schemas'

function makeRuntime(events: RunEvent[]): Runtime {
  let c = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `r${++c}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [{ kind: 'started', run_id: id, spec: s }, ...events.map(e => ({ ...e, run_id: id }))])
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || !q.length) return null
      return JSON.stringify(q.shift())
    },
    resumeRun() {},
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

describe('validate-spec example smoke test', () => {
  it('validateSpec returns ok for valid spec', () => {
    const r = validateSpec({ model: 'claude-3-5-sonnet', max_tokens: 1024, temperature: 0.7 })
    expect(r.ok).toBe(true)
  })

  it('validateSpec returns errors for missing model', () => {
    const r = validateSpec({ instructions: 'no model' })
    expect(r.ok).toBe(false)
    if (!r.ok) expect(r.errors.length).toBeGreaterThan(0)
  })

  it('buildSpec returns a spec object', () => {
    const s = buildSpec('claude', { maxTokens: 2048 })
    expect(s.model).toBe('claude')
    expect(s.max_tokens).toBe(2048)
  })
})

describe('multi-tool chaining example smoke test', () => {
  const convertTool = defineTool({
    name: 'convert',
    description: 'convert',
    schema: z.object({ amount: z.number(), from: z.string(), to: z.string() }),
    handler: ({ amount }) => amount * 0.92,
  })
  const formatTool = defineTool({
    name: 'format',
    description: 'format',
    schema: z.object({ value: z.number() }),
    handler: ({ value }) => value.toFixed(2),
  })

  it('createToolBridge registers all tools', () => {
    const bridge = createToolBridge(convertTool, formatTool)
    expect(bridge.registry.has('convert')).toBe(true)
    expect(bridge.registry.has('format')).toBe(true)
  })

  it('bridge dispatches convert tool', async () => {
    const bridge = new ToolBridge(new ToolRegistry().register(convertTool) as ToolRegistry)
    const rt = makeRuntime([
      { kind: 'tool_call', run_id: 'r', name: 'convert', input: '{"amount":100,"from":"USD","to":"EUR"}' },
      { kind: 'completed', run_id: 'r' },
    ])
    const agent = new Agent(rt)
    const events = []
    for await (const ev of bridge.run(agent.run(buildSpec('test')))) {
      events.push(ev)
    }
    const tr = events.find(e => e.kind === 'tool_result') as { kind: 'tool_result'; result: unknown } | undefined
    expect(tr?.result).toBeCloseTo(92)
  })
})

describe('error-handling example smoke test', () => {
  it('ToolBridge wraps thrown errors in result', async () => {
    const crashTool = defineTool({
      name: 'crash',
      description: 'crash',
      schema: z.object({}),
      handler: () => { throw new Error('boom') },
    })
    const bridge = new ToolBridge(new ToolRegistry().register(crashTool) as ToolRegistry)
    const rt = makeRuntime([
      { kind: 'tool_call', run_id: 'r', name: 'crash', input: '{}' },
      { kind: 'completed', run_id: 'r' },
    ])
    const events = []
    for await (const ev of bridge.run(new Agent(rt).run(buildSpec('test')))) {
      events.push(ev)
    }
    const tr = events.find(e => e.kind === 'tool_result') as { kind: 'tool_result'; result: unknown } | undefined
    expect((tr?.result as Record<string, unknown>)['error']).toContain('boom')
  })
})
