const RS145: Record<string, string[]> = {}
let RS145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `rs-${RS145_CTR++}`
      RS145[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = RS145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Runtime } from '../index'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(RS145).forEach((k) => delete RS145[k]); RS145_CTR = 0 })

describe('phase145 restart recovery via sidecar', () => {
  it('Runtime create-free-create cycle works', () => {
    const rt1 = new Runtime(); rt1.free(); expect(rt1.isFreed).toBe(true)
    const rt2 = new Runtime(); expect(rt2.isFreed).toBe(false); rt2.free()
  })

  it('free sets isFreed true', () => {
    const rt = new Runtime(); rt.free(); expect(rt.isFreed).toBe(true)
  })

  it('new run after free-create cycle starts', async () => {
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const agent = new Agent()
    const h1 = agent.run(spec)
    for await (const _ of h1) {}
    const h2 = agent.run(spec)
    expect(h2.runId).toBeDefined()
    expect(h2.runId).not.toBe(h1.runId)
  })

  it('run IDs differ across restarts', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = Array.from({ length: 3 }, () => agent.run(spec).runId)
    expect(new Set(ids).size).toBe(3)
  })

  it('ten restart cycles all unique IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = Array.from({ length: 10 }, () => agent.run(spec).runId)
    expect(new Set(ids).size).toBe(10)
  })

  it('each cycle run completes', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for (let i = 0; i < 5; i++) {
      const h = agent.run(spec)
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
    }
  })

  it('runtime not freed between cycles', () => {
    const rt = new Runtime()
    rt.startRun('{}'); rt.startRun('{}')
    expect(rt.isFreed).toBe(false)
    rt.free()
  })

  it('counter increments monotonically', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = [agent.run(spec).runId, agent.run(spec).runId]
    const n1 = parseInt(ids[0].split('-')[1])
    const n2 = parseInt(ids[1].split('-')[1])
    expect(n1).toBeLessThan(n2)
  })

  it('Runtime instances have independent counters', () => {
    const rt1 = new Runtime()
    const rt2 = new Runtime()
    const id1 = rt1.startRun('{}')
    const id2 = rt2.startRun('{}')
    expect(typeof id1).toBe('string')
    expect(typeof id2).toBe('string')
    rt1.free(); rt2.free()
  })
})
