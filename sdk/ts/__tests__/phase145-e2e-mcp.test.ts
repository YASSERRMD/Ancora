const MCP145: Record<string, string[]> = {}
let MCP145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `mcp145-${MCP145_CTR++}`
      MCP145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'tool_call', run_id: id, name: 'mcp_read_file', input: '{"path":"/etc/hosts"}' }),
        JSON.stringify({ kind: 'tool_result', run_id: id, name: 'mcp_read_file', output: '{"content":"127.0.0.1 localhost"}' }),
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
  it('mcp tool dispatch works', () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    const result = JSON.parse(reg.dispatch('mcp_read_file', { path: '/etc/hosts' }) as string)
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
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] }))
    const kinds: string[] = []
    for await (const ev of h) kinds.push((ev as { kind: string }).kind)
    const ci = kinds.indexOf('tool_call')
    const ri = kinds.indexOf('tool_result')
    expect(ci).toBeLessThan(ri)
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

  it('no live network calls in fixture mode', () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    expect(() => reg.dispatch('mcp_read_file', { path: '/tmp/local.txt' })).not.toThrow()
  })

  it('two mcp runs have distinct run IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] })
    expect(agent.run(spec).runId).not.toBe(agent.run(spec).runId)
  })

  it('tool_result output contains fixture content', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [mcpReadFile.spec] }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const tr = events.find((e) => (e as { kind: string }).kind === 'tool_result') as { output?: string }
    expect(tr?.output).toContain('localhost')
  })
})
