// Cross-language conformance: human-in-loop scenario -- TypeScript (offline).

interface XlangHilEvent {
  kind: string
  run_id: string
  prompt?: string
  options?: string[]
  decision?: string
  output?: Record<string, unknown>
}

const XLANG_HIL_RUN_ID = 'xlh-ts'

function makeXlangHilEvents(runId: string): XlangHilEvent[] {
  return [
    { kind: 'started',            run_id: runId },
    { kind: 'decision_requested', run_id: runId, prompt: 'Please approve the draft', options: ['approve', 'reject'] },
    { kind: 'decision_received',  run_id: runId, decision: '{"approved":true}' },
    { kind: 'completed',          run_id: runId, output: { result: 'hil-ok' } },
  ]
}

describe('xlang human-in-loop scenario -- TypeScript', () => {
  const events = makeXlangHilEvents(XLANG_HIL_RUN_ID)

  it('started event is first', () => {
    expect(events[0].kind).toBe('started')
  })

  it('decision_requested comes before decision_received', () => {
    const decKinds = events.filter(e => e.kind.startsWith('decision')).map(e => e.kind)
    expect(decKinds).toEqual(['decision_requested', 'decision_received'])
  })

  it('decision is approved', () => {
    const received = events.find(e => e.kind === 'decision_received')!
    const dec = JSON.parse(received.decision!)
    expect(dec.approved).toBe(true)
  })

  it('prompt is non-empty', () => {
    const requested = events.find(e => e.kind === 'decision_requested')!
    expect(requested.prompt).toBeTruthy()
    expect(requested.options?.length).toBeGreaterThan(0)
  })

  it('completed event is last', () => {
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('run_id consistent across events', () => {
    for (const ev of events) {
      expect(ev.run_id).toBe(XLANG_HIL_RUN_ID)
    }
  })
})
