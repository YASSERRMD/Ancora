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
          JSON.stringify({ kind: 'token', run_id: id, text: 'The ' }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'quick ' }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'fox' }),
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
import { collectEvents, tokenText } from '../helpers'
import { AgentSpecSchema } from '../schemas'

describe('tokenText with multi-token runs', () => {
  it('concatenates tokens in order', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(AgentSpecSchema.parse({ model: 'test' })))
    expect(tokenText(events)).toBe('The quick fox')
  })

  it('token count is 3', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(AgentSpecSchema.parse({ model: 'test' })))
    const tokens = events.filter((e) => e.kind === 'token')
    expect(tokens).toHaveLength(3)
  })
})
