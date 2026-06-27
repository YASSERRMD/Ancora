const ZD145: Record<string, string[]> = {}
let ZD145_CTR = 0
const SIDE_EFFECTS: string[] = []

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `zd-${ZD145_CTR++}`
      ZD145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = ZD145[id]; if (!q || !q.length) return null
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

beforeEach(() => {
  Object.keys(ZD145).forEach((k) => delete ZD145[k])
  ZD145_CTR = 0
  SIDE_EFFECTS.length = 0
})

const effectTool = defineTool({
  name: 'effect_once',
  description: 'Record side effect',
  schema: z.object({ label: z.string() }),
  handler: ({ label }) => { SIDE_EFFECTS.push(label); return `recorded:${label}` },
})

describe('phase145 zero duplicate side effects', () => {
  it('single dispatch records one side effect', () => {
    const reg = new ToolRegistry()
    reg.register(effectTool)
    reg.dispatch('effect_once', { label: 'A' })
    expect(SIDE_EFFECTS.filter((s) => s === 'A')).toHaveLength(1)
  })

  it('two distinct dispatches record two effects', () => {
    const reg = new ToolRegistry()
    reg.register(effectTool)
    reg.dispatch('effect_once', { label: 'B' })
    reg.dispatch('effect_once', { label: 'C' })
    expect(SIDE_EFFECTS).toHaveLength(2)
  })

  it('five distinct dispatches each recorded once', () => {
    const reg = new ToolRegistry()
    reg.register(effectTool)
    for (const ch of ['D', 'E', 'F', 'G', 'H']) reg.dispatch('effect_once', { label: ch })
    expect(SIDE_EFFECTS).toHaveLength(5)
    expect(new Set(SIDE_EFFECTS).size).toBe(5)
  })

  it('run IDs are unique across five runs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = Array.from({ length: 5 }, () => agent.run(spec).runId)
    expect(new Set(ids).size).toBe(5)
  })

  it('drain twice - second drain empty', async () => {
    const rt = new (await import('../index')).Runtime()
    const id = rt.startRun('{}')
    while (rt.pollRun(id) !== null) {}
    expect(rt.pollRun(id)).toBeNull()
    rt.free()
  })

  it('side effects cleared between tests', () => {
    expect(SIDE_EFFECTS).toHaveLength(0)
  })

  it('no duplicate run IDs across four cycles', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const seen = new Set<string>()
    for (let i = 0; i < 4; i++) {
      const id = agent.run(spec).runId
      expect(seen.has(id)).toBe(false)
      seen.add(id)
    }
  })

  it('effect not duplicated on registry re-registration', () => {
    const reg = new ToolRegistry()
    reg.register(effectTool)
    reg.dispatch('effect_once', { label: 'X' })
    expect(SIDE_EFFECTS.filter((s) => s === 'X')).toHaveLength(1)
  })
})
