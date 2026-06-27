interface JournalEntry { seq: number; kind: string; [key: string]: unknown }

const JOURNAL_FIXTURE: JournalEntry[] = [
  { seq: 0, kind: 'run_start', agent: 'ts-journal-agent' },
  { seq: 1, kind: 'tool_call', tool: 'noop' },
  { seq: 2, kind: 'tool_result', tool: 'noop', output: 'ok' },
  { seq: 3, kind: 'run_end' },
]

const RUNS145J: Record<string, string[]> = {}
let CTR145J = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `j145-${CTR145J++}`
      RUNS145J[id] = JOURNAL_FIXTURE.map((e) => JSON.stringify({ ...e, run_id: id }))
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = RUNS145J[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(RUNS145J).forEach((k) => delete RUNS145J[k]); CTR145J = 0 })

describe('phase145 journal matches core fixture', () => {
  it('fixture starts with run_start', () => {
    expect(JOURNAL_FIXTURE[0].kind).toBe('run_start')
  })

  it('fixture ends with run_end', () => {
    expect(JOURNAL_FIXTURE[JOURNAL_FIXTURE.length - 1].kind).toBe('run_end')
  })

  it('fixture seqs are contiguous', () => {
    const seqs = JOURNAL_FIXTURE.map((e) => e.seq)
    expect(seqs).toEqual([0, 1, 2, 3])
  })

  it('fixture has tool_call', () => {
    expect(JOURNAL_FIXTURE.some((e) => e.kind === 'tool_call')).toBe(true)
  })

  it('fixture has tool_result', () => {
    expect(JOURNAL_FIXTURE.some((e) => e.kind === 'tool_result')).toBe(true)
  })

  it('fixture JSON round-trips', () => {
    const rt = JSON.parse(JSON.stringify(JOURNAL_FIXTURE)) as JournalEntry[]
    expect(rt[0].kind).toBe('run_start')
    expect(rt[3].kind).toBe('run_end')
  })

  it('run events match fixture length', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'journal-agent' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect(events).toHaveLength(JOURNAL_FIXTURE.length)
  })

  it('run events have run_id', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'journal-agent' }))
    for await (const ev of h) {
      expect((ev as { run_id: string }).run_id).toBe(h.runId)
    }
  })

  it('run events kinds match fixture in order', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'journal-agent' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const kinds = events.map((e) => (e as { kind: string }).kind)
    expect(kinds).toEqual(JOURNAL_FIXTURE.map((e) => e.kind))
  })

  it('tool_result output is ok', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'journal-agent' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const tr = events.find((e) => (e as { kind: string }).kind === 'tool_result') as { output?: string }
    expect(tr?.output).toBe('ok')
  })
})
