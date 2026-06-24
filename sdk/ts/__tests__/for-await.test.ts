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
          JSON.stringify({ kind: 'token', run_id: id, text: 'x' }),
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
import { AgentSpecSchema } from '../schemas'

describe('for-await-of direct iteration', () => {
  it('works with for-await-of on agent.run() directly', async () => {
    const agent = new Agent()
    const kinds: string[] = []
    for await (const ev of agent.run(AgentSpecSchema.parse({ model: 'test' }))) {
      kinds.push(ev.kind)
    }
    expect(kinds).toEqual(['started', 'token', 'completed'])
  })

  it('break inside for-await-of stops iteration', async () => {
    const agent = new Agent()
    const events = []
    for await (const ev of agent.run(AgentSpecSchema.parse({ model: 'test' }))) {
      events.push(ev)
      if (ev.kind === 'started') break
    }
    expect(events).toHaveLength(1)
    expect(events[0].kind).toBe('started')
  })
})
