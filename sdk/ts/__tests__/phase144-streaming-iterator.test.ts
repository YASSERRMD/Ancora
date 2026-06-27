const ST_RUNS: Record<string, string[]> = {}
let ST_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `st-${ST_CTR++}`
      ST_RUNS[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'A' }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'B' }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'C' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = ST_RUNS[id]
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
  Object.keys(ST_RUNS).forEach((k) => delete ST_RUNS[k])
  ST_CTR = 0
})

describe('phase144 streaming async iterator', () => {
  it('for-await yields all 5 events', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    let count = 0
    for await (const _ of h) count++
    expect(count).toBe(5)
  })

  it('token events text concatenates to ABC', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    let text = ''
    for await (const ev of h) {
      if ((ev as { kind: string }).kind === 'token') {
        text += (ev as { text: string }).text
      }
    }
    expect(text).toBe('ABC')
  })

  it('first event is started', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    let first: unknown = null
    for await (const ev of h) {
      first = ev
      break
    }
    expect((first as { kind: string }).kind).toBe('started')
  })

  it('last event is completed', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('exactly 3 token events', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const tokens: unknown[] = []
    for await (const ev of h) {
      if ((ev as { kind: string }).kind === 'token') tokens.push(ev)
    }
    expect(tokens).toHaveLength(3)
  })

  it('handle is an async iterable', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(typeof h[Symbol.asyncIterator]).toBe('function')
  })

  it('two runs do not share token streams', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const h1 = agent.run(spec)
    const h2 = agent.run(spec)

    const tokens = (h: typeof h1) => {
      const out: string[] = []
      return (async () => {
        for await (const ev of h) {
          if ((ev as { kind: string }).kind === 'token') out.push((ev as { text: string }).text)
        }
        return out
      })()
    }

    const [t1, t2] = await Promise.all([tokens(h1), tokens(h2)])
    expect(t1).toEqual(['A', 'B', 'C'])
    expect(t2).toEqual(['A', 'B', 'C'])
    expect(t1).not.toBe(t2)
  })

  it('events contain run_id matching handle', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const runId = h.runId
    for await (const ev of h) {
      expect((ev as { run_id: string }).run_id).toBe(runId)
    }
  })

  it('handle.runId is set before iteration', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h.runId.length).toBeGreaterThan(0)
  })

  it('empty spec text field yields empty string accumulation', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3', instructions: '' })
    const h = agent.run(spec)
    let found = false
    for await (const ev of h) {
      if ((ev as { kind: string }).kind === 'token') {
        found = true
        expect(typeof (ev as { text: string }).text).toBe('string')
      }
    }
    expect(found).toBe(true)
  })
})
