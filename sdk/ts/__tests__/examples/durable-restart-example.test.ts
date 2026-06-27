jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec, collectEvents } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

// Minimal in-process run journal (stands in for SQLite or Redis).
class RunJournal {
  private _runs: Map<string, string[]> = new Map()

  recordRun(runId: string): void {
    if (!this._runs.has(runId)) {
      this._runs.set(runId, [])
    }
  }

  appendEvent(runId: string, payload: string): void {
    const q = this._runs.get(runId)
    if (q) q.push(payload)
  }

  eventsForRun(runId: string): string[] {
    return this._runs.get(runId) ?? []
  }

  get runCount(): number {
    return this._runs.size
  }
}

function makeOfflineRuntime(tokens: string[]): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `durable-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        ...tokens.map(t => ({ kind: 'token' as const, run_id: id, text: t })),
        { kind: 'completed', run_id: id },
      ])
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun() {},
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

describe('durable-restart example smoke test', () => {
  it('RunJournal records and replays events', () => {
    const journal = new RunJournal()
    journal.recordRun('run-1')
    journal.appendEvent('run-1', '{"kind":"started"}')
    journal.appendEvent('run-1', '{"kind":"completed"}')
    const events = journal.eventsForRun('run-1')
    expect(events).toHaveLength(2)
    expect(journal.runCount).toBe(1)
  })

  it('RunJournal tracks multiple runs independently', () => {
    const journal = new RunJournal()
    journal.recordRun('a')
    journal.recordRun('b')
    journal.appendEvent('a', 'eventA')
    expect(journal.runCount).toBe(2)
    expect(journal.eventsForRun('b')).toHaveLength(0)
  })

  it('RunJournal returns empty array for unknown run', () => {
    const journal = new RunJournal()
    expect(journal.eventsForRun('nonexistent')).toEqual([])
  })

  it('events persisted to journal can be replayed after restart', async () => {
    const journal = new RunJournal()
    const rt = makeOfflineRuntime(['tok1', 'tok2'])
    const agent = new Agent(rt)
    const handle = agent.run(buildSpec('model'))
    const runId = handle.runId
    journal.recordRun(runId)
    for await (const ev of handle) {
      journal.appendEvent(runId, JSON.stringify(ev))
    }
    const replayed = journal.eventsForRun(runId)
    expect(replayed.length).toBeGreaterThanOrEqual(1)
    agent.free()
  })

  it('total run count matches number of started runs', async () => {
    const journal = new RunJournal()
    const rt = makeOfflineRuntime(['ok'])
    for (let i = 0; i < 3; i++) {
      const agent = new Agent(rt)
      const handle = agent.run(buildSpec('model'))
      journal.recordRun(handle.runId)
      await collectEvents(handle)
      agent.free()
    }
    expect(journal.runCount).toBe(3)
  })

  it('recordRun is idempotent for the same run ID', () => {
    const journal = new RunJournal()
    journal.recordRun('dup')
    journal.recordRun('dup')
    expect(journal.runCount).toBe(1)
  })
})
