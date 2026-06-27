const CONC_RUNS: Record<string, string[]> = {}
let CONC_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `c-${CONC_CTR++}`
      CONC_RUNS[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = CONC_RUNS[id]
      if (!q || q.length === 0) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => {
  Object.keys(CONC_RUNS).forEach((k) => delete CONC_RUNS[k])
  CONC_CTR = 0
})

describe('phase144 concurrent runs', () => {
  it('two concurrent runs have distinct IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const h1 = agent.run(spec)
    const h2 = agent.run(spec)
    expect(h1.runId).not.toBe(h2.runId)
  })

  it('five concurrent runs have unique IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = Array.from({ length: 5 }, () => agent.run(spec).runId)
    expect(new Set(ids).size).toBe(5)
  })

  it('ten concurrent runs all unique', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = Array.from({ length: 10 }, () => agent.run(spec).runId)
    expect(new Set(ids).size).toBe(10)
  })

  it('two runs events are independent', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const h1 = agent.run(spec)
    const h2 = agent.run(spec)
    const ev1: unknown[] = []
    const ev2: unknown[] = []
    for await (const e of h1) ev1.push(e)
    for await (const e of h2) ev2.push(e)
    expect((ev1[0] as { run_id: string }).run_id).toBe(h1.runId)
    expect((ev2[0] as { run_id: string }).run_id).toBe(h2.runId)
  })

  it('draining one run does not exhaust another', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const h1 = agent.run(spec)
    const h2 = agent.run(spec)
    const ev1: unknown[] = []
    for await (const e of h1) ev1.push(e)
    expect(ev1.length).toBeGreaterThan(0)
    const ev2: unknown[] = []
    for await (const e of h2) ev2.push(e)
    expect(ev2.length).toBeGreaterThan(0)
  })

  it('concurrent runs produce started events with correct run_ids', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const handles = Array.from({ length: 3 }, () => agent.run(spec))
    for (const h of handles) {
      const ev = await h[Symbol.asyncIterator]().next()
      expect((ev.value as { run_id: string }).run_id).toBe(h.runId)
    }
  })

  it('run IDs monotonically increment in counter', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = [agent.run(spec).runId, agent.run(spec).runId, agent.run(spec).runId]
    const nums = ids.map((id) => parseInt(id.split('-')[1]))
    expect(nums[0]).toBeLessThan(nums[1])
    expect(nums[1]).toBeLessThan(nums[2])
  })

  it('parallel promise resolution all complete', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const drains = Array.from({ length: 5 }, () => {
      const h = agent.run(spec)
      return (async () => {
        const evs: unknown[] = []
        for await (const e of h) evs.push(e)
        return evs
      })()
    })
    const results = await Promise.all(drains)
    expect(results.every((r) => r.length > 0)).toBe(true)
  })

  it('last event of each concurrent run is completed', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for (let i = 0; i < 4; i++) {
      const h = agent.run(spec)
      const events: unknown[] = []
      for await (const e of h) events.push(e)
      expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
    }
  })
})
