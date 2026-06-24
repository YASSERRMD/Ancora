import * as path from 'path'

const MOCK_RUNS: Record<string, string[]> = {}
let runCounter = 0

const mockNativeModule = {
  Runtime: class MockNativeRuntime {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(specBytes: Buffer): string {
      const runId = `run-${runCounter++}`
      const spec = specBytes.toString('utf8')
      MOCK_RUNS[runId] = [
        JSON.stringify({ kind: 'started', run_id: runId, spec }),
        JSON.stringify({ kind: 'token', run_id: runId, text: 'Hello' }),
        JSON.stringify({ kind: 'token', run_id: runId, text: ' ' }),
        JSON.stringify({ kind: 'token', run_id: runId, text: 'world' }),
        JSON.stringify({ kind: 'completed', run_id: runId }),
      ]
      return runId
    }
    pollRun(runId: string): Buffer | null {
      const queue = MOCK_RUNS[runId]
      if (!queue || queue.length === 0) return null
      return Buffer.from(queue.shift()!, 'utf8')
    }
    resumeRun(runId: string, decision: Buffer): void {
      const queue = MOCK_RUNS[runId]
      if (!queue) return
      const dec = decision.toString('utf8')
      queue.push(JSON.stringify({ kind: 'resumed', run_id: runId, decision: dec }))
      queue.push(JSON.stringify({ kind: 'completed', run_id: runId }))
    }
  },
  version: () => '0.1.0',
}

jest.mock(path.join(__dirname, '..', 'ancora.node'), () => mockNativeModule, {
  virtual: true,
})

import { Runtime, version } from '../index'

beforeEach(() => {
  Object.keys(MOCK_RUNS).forEach((k) => delete MOCK_RUNS[k])
  runCounter = 0
})

describe('Runtime import', () => {
  it('exports Runtime class', () => {
    expect(typeof Runtime).toBe('function')
  })

  it('exports version function', () => {
    expect(typeof version).toBe('function')
  })
})

describe('Runtime constructor', () => {
  it('creates an instance without throwing', () => {
    const rt = new Runtime()
    expect(rt).toBeDefined()
  })

  it('starts with isFreed = false', () => {
    const rt = new Runtime()
    expect(rt.isFreed).toBe(false)
  })
})

describe('Runtime.startRun', () => {
  it('returns a non-empty run ID string', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{"model":"test"}')
    expect(typeof runId).toBe('string')
    expect(runId.length).toBeGreaterThan(0)
  })

  it('accepts Uint8Array spec', () => {
    const rt = new Runtime()
    const bytes = new TextEncoder().encode('{"model":"test"}')
    const runId = rt.startRun(bytes)
    expect(typeof runId).toBe('string')
    expect(runId.length).toBeGreaterThan(0)
  })

  it('different calls return different run IDs', () => {
    const rt = new Runtime()
    const id1 = rt.startRun('{"model":"a"}')
    const id2 = rt.startRun('{"model":"b"}')
    expect(id1).not.toBe(id2)
  })
})

describe('Runtime.pollRun', () => {
  it('first event is started', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    const raw = rt.pollRun(runId)
    expect(raw).not.toBeNull()
    const event = JSON.parse(raw!)
    expect(event.kind).toBe('started')
    expect(event.run_id).toBe(runId)
  })

  it('returns token events after started', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    rt.pollRun(runId) // started
    const ev = JSON.parse(rt.pollRun(runId)!)
    expect(ev.kind).toBe('token')
    expect(ev.text).toBe('Hello')
  })

  it('last event is completed', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    let last: string | null = null
    let ev = rt.pollRun(runId)
    while (ev !== null) {
      last = ev
      ev = rt.pollRun(runId)
    }
    expect(JSON.parse(last!).kind).toBe('completed')
  })

  it('returns null once events are exhausted', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    for (let i = 0; i < 10; i++) rt.pollRun(runId)
    expect(rt.pollRun(runId)).toBeNull()
  })

  it('produces exactly 5 events for a standard run', () => {
    const rt = new Runtime()
    const runId = rt.startRun('{}')
    const events: string[] = []
    let ev = rt.pollRun(runId)
    while (ev !== null) {
      events.push(ev)
      ev = rt.pollRun(runId)
    }
    expect(events).toHaveLength(5)
  })
})

describe('Runtime.free', () => {
  it('sets isFreed to true', () => {
    const rt = new Runtime()
    rt.free()
    expect(rt.isFreed).toBe(true)
  })

  it('isFreed is false before free()', () => {
    const rt = new Runtime()
    expect(rt.isFreed).toBe(false)
  })
})

describe('multiple run IDs', () => {
  it('two concurrent runs have independent event queues', () => {
    const rt = new Runtime()
    const id1 = rt.startRun('{"model":"a"}')
    const id2 = rt.startRun('{"model":"b"}')
    const ev1 = JSON.parse(rt.pollRun(id1)!)
    const ev2 = JSON.parse(rt.pollRun(id2)!)
    expect(ev1.run_id).toBe(id1)
    expect(ev2.run_id).toBe(id2)
  })

  it('draining one run does not affect another', () => {
    const rt = new Runtime()
    const id1 = rt.startRun('{}')
    const id2 = rt.startRun('{}')
    for (let i = 0; i < 10; i++) rt.pollRun(id1)
    expect(rt.pollRun(id1)).toBeNull()
    expect(rt.pollRun(id2)).not.toBeNull()
  })
})

describe('version()', () => {
  it('returns a semver-like string', () => {
    const v = version()
    expect(typeof v).toBe('string')
    expect(v).toMatch(/^\d+\.\d+\.\d+/)
  })
})
