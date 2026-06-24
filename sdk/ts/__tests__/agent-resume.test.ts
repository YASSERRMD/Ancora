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
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec: specBytes.toString('utf8') }),
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

const SPEC = AgentSpecSchema.parse({ model: 'test' })

describe('RunHandle.resume', () => {
  it('adds resumed event to the queue', async () => {
    const agent = new Agent()
    const handle = agent.run(SPEC)
    for await (const _ of handle.events()) { /* drain initial */ }
    handle.resume('approve')
    const events = []
    for await (const ev of handle.events()) {
      events.push(ev)
    }
    expect(events.some((e) => e.kind === 'resumed')).toBe(true)
  })

  it('resumed event has the correct decision', async () => {
    const agent = new Agent()
    const handle = agent.run(SPEC)
    for await (const _ of handle.events()) {}
    handle.resume('my-decision')
    const events = []
    for await (const ev of handle.events()) {
      events.push(ev)
    }
    const resumed = events.find((e) => e.kind === 'resumed')
    expect(resumed).toBeDefined()
    if (resumed?.kind === 'resumed') {
      expect(resumed.decision).toBe('my-decision')
    }
  })

  it('run() convenience: resume + drain returns events', async () => {
    const agent = new Agent()
    const handle = agent.run(SPEC)
    for await (const _ of handle.events()) {}
    const afterResume = await handle.run('go')
    expect(afterResume.some((e) => e.kind === 'completed')).toBe(true)
  })
})
