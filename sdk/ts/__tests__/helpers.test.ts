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
      resumeRun(): void {}
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { Agent } from '../agent'
import { collectEvents, tokenText } from '../helpers'
import { AgentSpecSchema } from '../schemas'

const SPEC = AgentSpecSchema.parse({ model: 'test' })

describe('collectEvents', () => {
  it('collects all events from an async iterable', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(SPEC))
    expect(events).toHaveLength(4)
  })

  it('last event is completed', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(SPEC))
    expect(events[events.length - 1].kind).toBe('completed')
  })
})

describe('tokenText', () => {
  it('concatenates token event texts', async () => {
    const agent = new Agent()
    const events = await collectEvents(agent.run(SPEC))
    expect(tokenText(events)).toBe('Hello world')
  })

  it('returns empty string when no tokens', () => {
    const events = [
      { kind: 'started' as const, run_id: 'r', spec: '{}' },
      { kind: 'completed' as const, run_id: 'r' },
    ]
    expect(tokenText(events)).toBe('')
  })
})
