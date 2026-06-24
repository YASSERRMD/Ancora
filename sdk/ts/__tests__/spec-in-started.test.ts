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

describe('started event contains the spec', () => {
  it('spec in started event matches the input spec', async () => {
    const spec = AgentSpecSchema.parse({ model: 'gpt-4', instructions: 'Be precise' })
    const agent = new Agent()
    const events = await collectEvents(agent.run(spec))
    const started = events.find((e) => e.kind === 'started')
    expect(started).toBeDefined()
    if (started?.kind === 'started') {
      const decoded = JSON.parse(started.spec)
      expect(decoded.model).toBe('gpt-4')
      expect(decoded.instructions).toBe('Be precise')
    }
  })
})
