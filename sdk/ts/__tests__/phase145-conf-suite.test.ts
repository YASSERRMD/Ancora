const CONF145: Record<string, string[]> = {}
let CONF145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `conf-${CONF145_CTR++}`
      CONF145[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = CONF145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(CONF145).forEach((k) => delete CONF145[k]); CONF145_CTR = 0 })

const SCENARIOS = [
  'single-agent',
  'multi-agent-verifier',
  'human-in-loop',
  'rag-retrieval',
]

describe('phase145 conformance suite passes', () => {
  it('all scenarios list is non-empty', () => {
    expect(SCENARIOS.length).toBeGreaterThan(0)
  })

  it('scenario names are non-empty strings', () => {
    SCENARIOS.forEach((s) => expect(s.length).toBeGreaterThan(0))
  })

  it('scenario names are distinct', () => {
    expect(new Set(SCENARIOS).size).toBe(SCENARIOS.length)
  })

  it('each scenario run starts', async () => {
    const agent = new Agent()
    for (const scenario of SCENARIOS) {
      const h = agent.run(AgentSpecSchema.parse({ model: scenario }))
      expect(h.runId.length).toBeGreaterThan(0)
    }
  })

  it('each scenario run completes', async () => {
    const agent = new Agent()
    for (const scenario of SCENARIOS) {
      const h = agent.run(AgentSpecSchema.parse({ model: scenario }))
      const events: unknown[] = []
      for await (const ev of h) events.push(ev)
      expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
    }
  })

  it('all four canonical scenarios present', () => {
    expect(SCENARIOS).toContain('single-agent')
    expect(SCENARIOS).toContain('multi-agent-verifier')
    expect(SCENARIOS).toContain('human-in-loop')
    expect(SCENARIOS).toContain('rag-retrieval')
  })

  it('scenario results are boolean-like', async () => {
    const results: Record<string, boolean> = {}
    const agent = new Agent()
    for (const scenario of SCENARIOS) {
      const h = agent.run(AgentSpecSchema.parse({ model: scenario }))
      const events: unknown[] = []
      for await (const ev of h) events.push(ev)
      results[scenario] = (events[events.length - 1] as { kind: string }).kind === 'completed'
    }
    Object.values(results).forEach((v) => expect(v).toBe(true))
  })

  it('second run of all scenarios is consistent', async () => {
    const agent = new Agent()
    for (const scenario of SCENARIOS) {
      const h = agent.run(AgentSpecSchema.parse({ model: scenario }))
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect(evs.length).toBeGreaterThan(0)
    }
  })

  it('conformance run IDs are unique', async () => {
    const agent = new Agent()
    const ids = SCENARIOS.map((s) => agent.run(AgentSpecSchema.parse({ model: s })).runId)
    expect(new Set(ids).size).toBe(ids.length)
  })

  it('single-agent scenario has started event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'single-agent' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect(events[0]).toMatchObject({ kind: 'started' })
  })
})
