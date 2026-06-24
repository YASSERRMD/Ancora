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

import { Runtime } from '../index'
import { AgentSpecSchema } from '../schemas'
import { encodeSpec, parseEvent } from '../wire'

describe('spec-to-runtime integration', () => {
  it('a Zod-validated spec can be encoded and passed to startRun', () => {
    const spec = AgentSpecSchema.parse({ model: 'gpt-4', instructions: 'hello' })
    const bytes = encodeSpec(spec)
    const rt = new Runtime()
    const runId = rt.startRun(bytes)
    expect(typeof runId).toBe('string')
  })

  it('started event spec field matches the encoded spec', () => {
    const spec = AgentSpecSchema.parse({ model: 'gpt-4', instructions: 'hello' })
    const bytes = encodeSpec(spec)
    const rt = new Runtime()
    const runId = rt.startRun(bytes)
    const raw = rt.pollRun(runId)!
    const ev = parseEvent(raw)
    expect(ev.kind).toBe('started')
    if (ev.kind === 'started') {
      const decoded = JSON.parse(ev.spec)
      expect(decoded.model).toBe('gpt-4')
    }
  })

  it('run completes after polling all events', () => {
    const spec = AgentSpecSchema.parse({ model: 'test' })
    const rt = new Runtime()
    const runId = rt.startRun(encodeSpec(spec))
    const events: string[] = []
    let ev = rt.pollRun(runId)
    while (ev !== null) {
      events.push(ev)
      ev = rt.pollRun(runId)
    }
    const last = parseEvent(events[events.length - 1])
    expect(last.kind).toBe('completed')
  })
})
