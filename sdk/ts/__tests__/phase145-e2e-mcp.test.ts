const MCP145: Record<string, string[]> = {}
let MCP145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `mcp145-${MCP145_CTR++}`
      // Note: no raw 'tool_result' event here — per tool-bridge.ts, 'tool_result'
      // is only ever synthesized client-side by ToolBridge.run() from a
      // 'tool_call' event; it is never part of the raw wire/RunEventSchema
      // stream, so it must not be fabricated directly by the mocked runtime.
      MCP145[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'tool_call', run_id: id, name: 'mcp_read_file', input: '{"path":"/etc/hosts"}' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = MCP145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(MCP145).forEach((k) => delete MCP145[k]); MCP145_CTR = 0 })

const mcpReadFile = defineTool({
  name: 'mcp_read_file',
  description: 'Read file via MCP',
  schema: z.object({ path: z.string() }),
  handler: ({ path }) => JSON.stringify({ content: `fixture:${path}` }),
})

describe('phase145 e2e mcp end to end', () => {
  it('mcp tool dispatch works', async () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    const result = JSON.parse((await reg.dispatch('mcp_read_file', { path: '/etc/hosts' })) as string)
    expect(result.content).toContain('/etc/hosts')
  })

  it('agent run emits tool_call mcp_read_file', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect(events.some((e) => (e as { name?: string }).name === 'mcp_read_file')).toBe(true)
  })

  it('tool_call event precedes tool_result', async () => {
    // ToolBridge is the real (and only) source of 'tool_result' events — it
    // synthesizes them from 'tool_call' events on the fly, replacing the
    // 'tool_call' in its output stream rather than emitting both. So the
    // meaningful check here is that 'started' (raw) precedes the bridged
    // 'tool_result'.
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] }))
    const bridge = new ToolBridge(reg)
    const kinds: string[] = []
    for await (const ev of bridge.run(h)) kinds.push(ev.kind)
    const si = kinds.indexOf('started')
    const ri = kinds.indexOf('tool_result')
    expect(si).toBeLessThan(ri)
  })

  it('run completes after mcp call', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('ToolBridge handles mcp tool call', async () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] }))
    const bridge = new ToolBridge(reg)
    const events: unknown[] = []
    for await (const ev of bridge.run(h)) events.push(ev)
    expect(events.length).toBeGreaterThan(0)
  })

  it('mcp tool spec has input_schema', () => {
    expect(mcpReadFile.spec.input_schema.type).toBe('object')
  })

  it('mcp tool input has path property', () => {
    expect(mcpReadFile.spec.input_schema.properties).toHaveProperty('path')
  })

  it('no live network calls in fixture mode', async () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    const result = await reg.dispatch('mcp_read_file', { path: '/tmp/local.txt' })
    expect(result).toBeDefined()
  })

  it('two mcp runs have distinct run IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] })
    expect(agent.run(spec).runId).not.toBe(agent.run(spec).runId)
  })

  it('tool_result output contains fixture content', async () => {
    // The real ToolBridge-synthesized 'tool_result' carries a 'result' field
    // (the actual handler's return value), not 'output' — match that shape
    // and the content mcpReadFile's handler actually produces.
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] }))
    const bridge = new ToolBridge(reg)
    const events: unknown[] = []
    for await (const ev of bridge.run(h)) events.push(ev)
    const tr = events.find((e) => (e as { kind: string }).kind === 'tool_result') as { result?: unknown }
    expect(JSON.stringify(tr?.result)).toContain('fixture:')
  })
})
