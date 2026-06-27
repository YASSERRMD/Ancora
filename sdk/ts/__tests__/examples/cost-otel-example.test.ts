jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec, collectEvents } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

// Minimal stand-in for an OTEL span.
interface SpanRecord {
  name: string
  durationMs: number
  attrs: Record<string, unknown>
}

class Span {
  private _start = Date.now()
  readonly name: string
  readonly attrs: Record<string, unknown> = {}

  constructor(name: string) {
    this.name = name
  }

  setAttribute(key: string, value: unknown): void {
    this.attrs[key] = value
  }

  end(): SpanRecord {
    return { name: this.name, durationMs: Date.now() - this._start, attrs: { ...this.attrs } }
  }
}

function estimateTokens(text: string): number {
  return Math.max(1, Math.ceil(text.length / 4))
}

function makeOfflineRuntime(tokens: string[]): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `cost-${++counter}`
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

describe('cost-otel example smoke test', () => {
  it('Span records name and attributes', () => {
    const s = new Span('agent.run')
    s.setAttribute('run.id', 'abc')
    const rec = s.end()
    expect(rec.name).toBe('agent.run')
    expect(rec.attrs['run.id']).toBe('abc')
  })

  it('Span durationMs is non-negative', () => {
    const s = new Span('test')
    const rec = s.end()
    expect(rec.durationMs).toBeGreaterThanOrEqual(0)
  })

  it('estimateTokens returns 1 for empty string', () => {
    expect(estimateTokens('')).toBe(1)
  })

  it('estimateTokens returns ceil of length/4', () => {
    expect(estimateTokens('abcd')).toBe(1)
    expect(estimateTokens('abcde')).toBe(2)
    expect(estimateTokens('a'.repeat(100))).toBe(25)
  })

  it('cost spans accumulate over a real run', async () => {
    const rt = makeOfflineRuntime(['Hello', ' world', '!'])
    const agent = new Agent(rt)

    const root = new Span('agent.run')
    const events = await collectEvents(agent.run(buildSpec('model')))

    let totalTokens = 0
    for (const ev of events) {
      if (ev.kind === 'token') {
        totalTokens += estimateTokens(ev.text)
      }
    }
    root.setAttribute('event.count', events.length)
    root.setAttribute('tokens.estimated', totalTokens)
    const rec = root.end()

    expect(rec.attrs['event.count']).toBe(events.length)
    expect(typeof rec.attrs['tokens.estimated']).toBe('number')
    agent.free()
  })

  it('summary span reports correct token total', async () => {
    const rt = makeOfflineRuntime(['a'.repeat(20), 'b'.repeat(40)])
    const agent = new Agent(rt)
    const events = await collectEvents(agent.run(buildSpec('model')))
    const tokenEvents = events.filter(e => e.kind === 'token')
    const total = tokenEvents.reduce((sum, e) => sum + estimateTokens((e as any).text ?? ''), 0)
    const summary = new Span('agent.summary')
    summary.setAttribute('tokens.estimated', total)
    const rec = summary.end()
    expect(rec.attrs['tokens.estimated']).toBe(total)
    agent.free()
  })
})
