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
          JSON.stringify({ kind: 'token', run_id: id, text: ' ' }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'world' }),
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

function drainRun(rt: Runtime, runId: string): string[] {
  const events: string[] = []
  let ev = rt.pollRun(runId)
  while (ev !== null) {
    events.push(ev)
    ev = rt.pollRun(runId)
  }
  return events
}

describe('event format', () => {
  it('all events are valid JSON', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    const events = drainRun(rt, runId)
    for (const raw of events) {
      expect(() => JSON.parse(raw)).not.toThrow()
    }
  })

  it('all events have kind field', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    const events = drainRun(rt, runId).map((e) => JSON.parse(e))
    for (const ev of events) {
      expect(typeof ev.kind).toBe('string')
    }
  })

  it('all events have matching run_id', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    const events = drainRun(rt, runId).map((e) => JSON.parse(e))
    for (const ev of events) {
      expect(ev.run_id).toBe(runId)
    }
  })

  it('started event contains spec', () => {
    const rt = new Runtime()
    const spec = '{"model":"gpt-4"}'
    const runId = rt.startRun(spec)
    const first = JSON.parse(rt.pollRun(runId)!)
    expect(first.kind).toBe('started')
    expect(first.spec).toBe(spec)
  })

  it('token events have text field', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    rt.pollRun(runId)
    const ev = JSON.parse(rt.pollRun(runId)!)
    expect(ev.kind).toBe('token')
    expect(typeof ev.text).toBe('string')
  })

  it('unknown run ID returns null without throwing', () => {
    const rt = new Runtime()
    expect(() => rt.pollRun('nonexistent-run-id')).not.toThrow()
    expect(rt.pollRun('nonexistent-run-id')).toBeNull()
  })
})
