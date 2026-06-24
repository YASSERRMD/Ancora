jest.mock('../ancora.node', () => {
  const runs: Record<string, string[]> = {}
  let counter = 0
  return {
    Runtime: class MockRuntime {
      private _freed = false
      get isFreed(): boolean { return this._freed }
      free(): void { this._freed = true }
      startRun(specBytes: Buffer): string {
        if (this._freed) throw new Error('Runtime has been freed')
        const id = `run-${counter++}`
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec: specBytes.toString('utf8') }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(runId: string): Buffer | null {
        if (this._freed) throw new Error('Runtime has been freed')
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

describe('Runtime lifecycle', () => {
  it('can start and drain runs before freeing', () => {
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id)
    rt.pollRun(id)
    expect(() => rt.free()).not.toThrow()
    expect(rt.isFreed).toBe(true)
  })

  it('startRun throws after free', () => {
    const rt = new Runtime()
    rt.free()
    expect(() => rt.startRun('{}')).toThrow()
  })

  it('pollRun throws after free', () => {
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.free()
    expect(() => rt.pollRun(id)).toThrow()
  })

  it('multiple instances are independent', () => {
    const rt1 = new Runtime()
    const rt2 = new Runtime()
    rt1.free()
    expect(rt1.isFreed).toBe(true)
    expect(rt2.isFreed).toBe(false)
  })
})
