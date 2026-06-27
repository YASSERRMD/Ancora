const RUNS144: Record<string, string[]> = {}
let CTR144 = 0

jest.mock('../ancora.node', () => ({
  Runtime: class MockRuntime144 {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(specBytes: Buffer): string {
      const id = `r144-${CTR144++}`
      RUNS144[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'hello' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(runId: string): Buffer | null {
      const q = RUNS144[runId]
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
  Object.keys(RUNS144).forEach((k) => delete RUNS144[k])
  CTR144 = 0
})

describe('phase144 single agent run', () => {
  it('run handle is defined', () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(handle).toBeDefined()
  })

  it('run handle has runId', () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(typeof handle.runId).toBe('string')
    expect(handle.runId.length).toBeGreaterThan(0)
  })

  it('two consecutive runs have distinct run IDs', () => {
    const agent = new Agent()
    const h1 = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const h2 = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h1.runId).not.toBe(h2.runId)
  })

  it('first event from run is started', async () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of handle) {
      events.push(ev)
    }
    expect((events[0] as { kind: string }).kind).toBe('started')
  })

  it('last event from run is completed', async () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of handle) {
      events.push(ev)
    }
    const last = events[events.length - 1] as { kind: string }
    expect(last.kind).toBe('completed')
  })

  it('run produces exactly 3 events', async () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of handle) {
      events.push(ev)
    }
    expect(events).toHaveLength(3)
  })

  it('token event has text field', async () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of handle) {
      events.push(ev)
    }
    const token = events.find((e) => (e as { kind: string }).kind === 'token') as { text: string } | undefined
    expect(token?.text).toBe('hello')
  })

  it('run is iterable via for-await', async () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    let count = 0
    for await (const _ of handle) {
      count++
    }
    expect(count).toBe(3)
  })

  it('run handle is an async iterable', () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(typeof handle[Symbol.asyncIterator]).toBe('function')
  })

  it('run started event has run_id matching handle', async () => {
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const runId = handle.runId
    const events: unknown[] = []
    for await (const ev of handle) {
      events.push(ev)
    }
    const started = events[0] as { run_id: string }
    expect(started.run_id).toBe(runId)
  })
})
