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
          JSON.stringify({ kind: 'token', run_id: id, text: 'Hi' }),
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

import { runOnce } from '../helpers'
import { AgentSpecSchema } from '../schemas'

describe('runOnce', () => {
  it('returns all events for a single run', async () => {
    const events = await runOnce(AgentSpecSchema.parse({ model: 'test' }))
    expect(events.length).toBe(3)
  })

  it('last event is completed', async () => {
    const events = await runOnce(AgentSpecSchema.parse({ model: 'test' }))
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('can be called multiple times independently', async () => {
    const spec = AgentSpecSchema.parse({ model: 'test' })
    const [evs1, evs2] = await Promise.all([runOnce(spec), runOnce(spec)])
    expect(evs1[evs1.length - 1].kind).toBe('completed')
    expect(evs2[evs2.length - 1].kind).toBe('completed')
  })
})
