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

import { Runtime } from '../index'

describe('Runtime.resumeRun', () => {
  it('adds resumed event after exhausting initial events', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    rt.pollRun(runId)
    rt.pollRun(runId)
    rt.resumeRun(runId, 'approve')
    const ev = JSON.parse(rt.pollRun(runId)!)
    expect(ev.kind).toBe('resumed')
    expect(ev.decision).toBe('approve')
  })

  it('resumed event is followed by completed', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    rt.pollRun(runId)
    rt.pollRun(runId)
    rt.resumeRun(runId, 'go')
    rt.pollRun(runId)
    const ev = JSON.parse(rt.pollRun(runId)!)
    expect(ev.kind).toBe('completed')
  })

  it('accepts Uint8Array decision', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    rt.pollRun(runId)
    rt.pollRun(runId)
    expect(() => rt.resumeRun(runId, Buffer.from('deny', 'utf8'))).not.toThrow()
  })
})
