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
          JSON.stringify({ kind: 'token', run_id: id, text: 'A' }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'B' }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'C' }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(runId: string): Buffer | null {
        const q = runs[runId]
        if (!q || q.length === 0) return null
        return Buffer.from(q.shift()!, 'utf8')
      }
      resumeRun(): void {}
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { Agent } from '../agent'
import { collectEvents } from '../helpers'
import { AgentSpecSchema } from '../schemas'

const SPEC = AgentSpecSchema.parse({ model: 'test' })

describe('event type narrowing', () => {
  it('started event has spec field', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(SPEC))
    const started = events.find((e) => e.kind === 'started')
    expect(started).toBeDefined()
    if (started?.kind === 'started') {
      expect(typeof started.spec).toBe('string')
    }
  })

  it('token events have text field', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(SPEC))
    const tokens = events.filter((e) => e.kind === 'token')
    for (const t of tokens) {
      if (t.kind === 'token') expect(typeof t.text).toBe('string')
    }
  })

  it('events are ordered: started, tokens, completed', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(SPEC))
    expect(events[0].kind).toBe('started')
    for (let i = 1; i < events.length - 1; i++) {
      expect(events[i].kind).toBe('token')
    }
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('run_id is consistent across all events', async () => {
    const agent = new Agent()
    const handle = agent.run(SPEC)
    const events = await collectEvents(handle)
    const ids = events.map((e) => e.run_id)
    expect(new Set(ids).size).toBe(1)
    expect(ids[0]).toBe(handle.runId)
  })

  it('exactly 5 events for a standard run', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(SPEC))
    expect(events).toHaveLength(5)
  })
})
