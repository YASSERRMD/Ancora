jest.mock('../ancora.node', () => {
  const runs: Record<string, string[]> = {}
  let counter = 0
  return {
    Runtime: class MockRuntime {
      private _freed = false
      get isFreed(): boolean { return this._freed }
      free(): void { this._freed = true }
      startRun(specBytes: Buffer): string {
        if (this._freed) throw new Error('freed')
        const id = `run-${counter++}`
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec: specBytes.toString('utf8') }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(runId: string): Buffer | null {
        if (this._freed) throw new Error('freed')
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
import { Runtime } from '../index'
import { AgentSpecSchema } from '../schemas'
import { collectEvents } from '../helpers'

const SPEC = AgentSpecSchema.parse({ model: 'test' })

describe('Agent lifecycle', () => {
  it('isFreed is false initially', () => {
    const agent = new Agent()
    expect(agent.isFreed).toBe(false)
  })

  it('isFreed is true after free()', () => {
    const agent = new Agent()
    agent.free()
    expect(agent.isFreed).toBe(true)
  })

  it('accepts an injected Runtime', async () => {
    const rt = new Runtime()
    const agent = new Agent(rt)
    const events = await collectEvents(agent.run(SPEC))
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('run() after free() throws', () => {
    const agent = new Agent()
    agent.free()
    expect(() => agent.run(SPEC)).toThrow()
  })
})
