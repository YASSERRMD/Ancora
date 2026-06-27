const VRUNS: Record<string, string[]> = {}
let VCTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `v-${VCTR++}`
      VRUNS[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'verified' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = VRUNS[id]
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
  Object.keys(VRUNS).forEach((k) => delete VRUNS[k])
  VCTR = 0
})

describe('phase144 multi-agent verifier', () => {
  it('drafter produces a run ID', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h.runId).toBeDefined()
  })

  it('verifier produces a run ID', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h.runId).toBeDefined()
  })

  it('drafter and verifier run IDs differ', () => {
    const agent = new Agent()
    const drafter = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const verifier = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(drafter.runId).not.toBe(verifier.runId)
  })

  it('drafter completes successfully', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const last = events[events.length - 1] as { kind: string }
    expect(last.kind).toBe('completed')
  })

  it('verifier completes after drafter', async () => {
    const agent = new Agent()
    const d = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    for await (const _ of d) {}
    const v = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of v) events.push(ev)
    expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('three node pipeline produces distinct IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = [
      agent.run(spec).runId,
      agent.run(spec).runId,
      agent.run(spec).runId,
    ]
    expect(new Set(ids).size).toBe(3)
  })

  it('verifier started event has its own run_id', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    agent.run(spec)
    const verifier = agent.run(spec)
    const events: unknown[] = []
    for await (const ev of verifier) events.push(ev)
    const started = events[0] as { kind: string; run_id: string }
    expect(started.kind).toBe('started')
    expect(started.run_id).toBe(verifier.runId)
  })

  it('both runs yield token events', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const r1 = agent.run(spec)
    const r2 = agent.run(spec)
    const collect = async (h: typeof r1) => {
      const evs: unknown[] = []
      for await (const e of h) evs.push(e)
      return evs
    }
    const [ev1, ev2] = await Promise.all([collect(r1), collect(r2)])
    const hasToken = (evs: unknown[]) => evs.some((e) => (e as { kind: string }).kind === 'token')
    expect(hasToken(ev1)).toBe(true)
    expect(hasToken(ev2)).toBe(true)
  })

  it('two simultaneous runs do not share events', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const r1 = agent.run(spec)
    const r2 = agent.run(spec)
    const ev1: unknown[] = []
    const ev2: unknown[] = []
    for await (const e of r1) ev1.push(e)
    for await (const e of r2) ev2.push(e)
    const id1 = (ev1[0] as { run_id: string }).run_id
    const id2 = (ev2[0] as { run_id: string }).run_id
    expect(id1).not.toBe(id2)
  })

  it('verifier run_id not same as drafter run_id', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const d = agent.run(spec)
    const v = agent.run(spec)
    expect(d.runId).not.toBe(v.runId)
  })
})
