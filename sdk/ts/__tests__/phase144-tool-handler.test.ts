jest.mock('../ancora.node', () => {
  const runs: Record<string, string[]> = {}
  let ctr = 0
  return {
    Runtime: class {
      private _freed = false
      get isFreed(): boolean { return this._freed }
      free(): void { this._freed = true }
      startRun(_: Buffer): string {
        const id = `th-${ctr++}`
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id }),
          JSON.stringify({ kind: 'tool_call', run_id: id, name: 'greet', input: '{"name":"world"}' }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(id: string): Buffer | null {
        const q = runs[id]
        if (!q || q.length === 0) return null
        return Buffer.from(q.shift()!, 'utf8')
      }
      resumeRun(): void {}
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

const greetTool = defineTool({
  name: 'greet',
  description: 'Greet someone',
  schema: z.object({ name: z.string() }),
  handler: ({ name }) => `Hello, ${name}!`,
})

describe('phase144 tool handler execution', () => {
  it('defineTool creates a tool with correct name', () => {
    expect(greetTool.name).toBe('greet')
  })

  it('defineTool creates a tool with spec', () => {
    expect(greetTool.spec).toBeDefined()
    expect(greetTool.spec.name).toBe('greet')
  })

  it('ToolRegistry.register adds tool', () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    expect(reg.get('greet')).toBeDefined()
  })

  it('ToolRegistry.dispatch calls handler', () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    const result = reg.dispatch('greet', { name: 'world' })
    expect(result).toBe('Hello, world!')
  })

  it('ToolBridge processes tool_call event', async () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'test', tools: [greetTool.spec] }))
    const bridge = new ToolBridge(reg)
    const events: unknown[] = []
    for await (const ev of bridge.run(handle)) {
      events.push(ev)
    }
    expect(events.length).toBeGreaterThan(0)
  })

  it('handler receives parsed input', () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    const result = reg.dispatch('greet', { name: 'Ancora' })
    expect(result).toContain('Ancora')
  })

  it('tool spec has description', () => {
    expect(greetTool.spec.description).toBe('Greet someone')
  })

  it('tool spec has input_schema', () => {
    expect(greetTool.spec.input_schema).toBeDefined()
    expect(greetTool.spec.input_schema.type).toBe('object')
  })

  it('ToolRegistry returns undefined for unknown tool', () => {
    const reg = new ToolRegistry()
    expect(reg.get('unknown')).toBeUndefined()
  })

  it('multiple tools can be registered', () => {
    const t2 = defineTool({
      name: 'farewell',
      description: 'Farewell',
      schema: z.object({ name: z.string() }),
      handler: ({ name }) => `Goodbye, ${name}!`,
    })
    const reg = new ToolRegistry()
    reg.register(greetTool)
    reg.register(t2)
    expect(reg.get('greet')).toBeDefined()
    expect(reg.get('farewell')).toBeDefined()
  })
})
