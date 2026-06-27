const PERF145: Record<string, string[]> = {}
let PERF145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `p-${PERF145_CTR++}`
      PERF145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = PERF145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(PERF145).forEach((k) => delete PERF145[k]); PERF145_CTR = 0 })

const noopTool = defineTool({
  name: 'noop_perf',
  description: 'noop for perf',
  schema: z.object({ x: z.number() }),
  handler: ({ x }) => x,
})

describe('phase145 call overhead benchmark', () => {
  it('1000 tool dispatches under 5 seconds', () => {
    const reg = new ToolRegistry()
    reg.register(noopTool)
    const start = Date.now()
    for (let i = 0; i < 1000; i++) reg.dispatch('noop_perf', { x: i })
    expect(Date.now() - start).toBeLessThan(5000)
  })

  it('100 tool dispatches under 1 second', () => {
    const reg = new ToolRegistry()
    reg.register(noopTool)
    const start = Date.now()
    for (let i = 0; i < 100; i++) reg.dispatch('noop_perf', { x: i })
    expect(Date.now() - start).toBeLessThan(1000)
  })

  it('10 run starts under 5 seconds', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const start = Date.now()
    for (let i = 0; i < 10; i++) {
      const h = agent.run(spec)
      for await (const _ of h) {}
    }
    expect(Date.now() - start).toBeLessThan(5000)
  })

  it('dispatch is deterministic', () => {
    const reg = new ToolRegistry()
    reg.register(noopTool)
    expect(reg.dispatch('noop_perf', { x: 42 })).toBe(42)
    expect(reg.dispatch('noop_perf', { x: 42 })).toBe(42)
  })

  it('500 dispatches via registry under 5 seconds', () => {
    const reg = new ToolRegistry()
    reg.register(noopTool)
    const start = Date.now()
    for (let i = 0; i < 500; i++) reg.dispatch('noop_perf', { x: i })
    expect(Date.now() - start).toBeLessThan(5000)
  })

  it('run ID creation is fast', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const start = Date.now()
    for (let i = 0; i < 50; i++) agent.run(spec)
    expect(Date.now() - start).toBeLessThan(1000)
  })

  it('tool dispatch throughput result types are correct', () => {
    const reg = new ToolRegistry()
    reg.register(noopTool)
    for (let i = 0; i < 10; i++) {
      const r = reg.dispatch('noop_perf', { x: i })
      expect(typeof r).toBe('number')
    }
  })
})
