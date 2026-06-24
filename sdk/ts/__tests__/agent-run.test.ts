jest.mock('../ancora.node', () => {
  const runs: Record<string, string[]> = {}
  let counter = 0
  return {
    Runtime: class MockRuntime {
      private _freed = false
      get isFreed(): boolean { return this._freed }
      free(): void { this._freed = true }
      startRun(specBytes: Buffer): string {
        const id = `run-${counter++}`
        const spec = specBytes.toString('utf8')
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'Hello' }),
          JSON.stringify({ kind: 'token', run_id: id, text: ' world' }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(runId: string): Buffer | null {
        const q = runs[runId]
        if (!q || q.length === 0) return null
        return Buffer.from(q.shift()!, 'utf8')
      }
      resumeRun(runId: string, decision: Buffer): void {
        const q = runs[runId]
        if (!q) return
        q.push(JSON.stringify({ kind: 'resumed', run_id: runId, decision: decision.toString('utf8') }))
        q.push(JSON.stringify({ kind: 'completed', run_id: runId }))
      }
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

function makeSpec(model = 'gpt-4') {
  return AgentSpecSchema.parse({ model })
}

beforeEach(() => {
  jest.clearAllMocks()
})

describe('Agent class', () => {
  it('constructs without arguments', () => {
    const agent = new Agent()
    expect(agent).toBeDefined()
    expect(agent.isFreed).toBe(false)
  })

  it('free sets isFreed to true', () => {
    const agent = new Agent()
    agent.free()
    expect(agent.isFreed).toBe(true)
  })
})

describe('Agent.run', () => {
  it('returns a RunHandle with a runId', () => {
    const agent = new Agent()
    const handle = agent.run(makeSpec())
    expect(typeof handle.runId).toBe('string')
    expect(handle.runId.length).toBeGreaterThan(0)
  })

  it('single agent run completes', async () => {
    const agent = new Agent()
    const handle = agent.run(makeSpec())
    const events = []
    for await (const ev of handle) {
      events.push(ev)
    }
    const last = events[events.length - 1]
    expect(last.kind).toBe('completed')
  })

  it('first event is started', async () => {
    const agent = new Agent()
    const events = []
    for await (const ev of agent.run(makeSpec())) {
      events.push(ev)
    }
    expect(events[0].kind).toBe('started')
  })

  it('async iteration yields token events', async () => {
    const agent = new Agent()
    const events = []
    for await (const ev of agent.run(makeSpec())) {
      events.push(ev)
    }
    const tokens = events.filter((e) => e.kind === 'token')
    expect(tokens.length).toBeGreaterThan(0)
  })
})

describe('RunHandle.events()', () => {
  it('yields all events when called explicitly', async () => {
    const agent = new Agent()
    const handle = agent.run(makeSpec())
    const events = []
    for await (const ev of handle.events()) {
      events.push(ev)
    }
    expect(events.length).toBe(4)
  })

  it('returns completed as last event', async () => {
    const agent = new Agent()
    const handle = agent.run(makeSpec())
    let last = null
    for await (const ev of handle.events()) {
      last = ev
    }
    expect(last?.kind).toBe('completed')
  })
})
