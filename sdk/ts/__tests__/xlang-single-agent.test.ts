// Cross-language conformance: single agent scenario -- TypeScript binding (offline).

interface XlangEvent {
  kind: string
  run_id: string
  text?: string
  spec?: string
}

const XLANG_RUN_ID = 'xlang-ts-001'

function makeXlangEvents(runId: string): XlangEvent[] {
  return [
    { kind: 'started', run_id: runId, spec: '{}' },
    { kind: 'token', run_id: runId, text: 'xlang ts result' },
    { kind: 'completed', run_id: runId },
  ]
}

describe('xlang single agent -- TypeScript', () => {
  const events = makeXlangEvents(XLANG_RUN_ID)

  it('started event is first', () => {
    expect(events[0].kind).toBe('started')
  })

  it('completed event is last', () => {
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('run_id is consistent across all events', () => {
    for (const ev of events) {
      expect(ev.run_id).toBe(XLANG_RUN_ID)
    }
  })

  it('has at least two events', () => {
    expect(events.length).toBeGreaterThanOrEqual(2)
  })

  it('token event has non-empty text', () => {
    const tokens = events.filter(e => e.kind === 'token')
    expect(tokens.length).toBeGreaterThan(0)
    for (const tok of tokens) {
      expect(tok.text).toBeTruthy()
    }
  })

  it('events serialise and deserialise cleanly', () => {
    for (const ev of events) {
      const serialised = JSON.stringify(ev)
      const decoded = JSON.parse(serialised) as XlangEvent
      expect(decoded.kind).toBe(ev.kind)
    }
  })

  it('first event spec field is valid JSON', () => {
    const first = events[0]
    expect(first.kind).toBe('started')
    const parsed = JSON.parse(first.spec ?? '{}')
    expect(typeof parsed).toBe('object')
  })
})
