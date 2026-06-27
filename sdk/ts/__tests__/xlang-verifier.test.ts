// Cross-language conformance: verifier scenario -- TypeScript (offline).

interface XlangVerifierEvent {
  kind: string
  run_id: string
  activity_key?: string
  output?: { verdict: string }
}

const XLANG_VERIFIER_RUN_ID = 'xlv-ts'

function makeXlangVerifierEvents(runId: string): XlangVerifierEvent[] {
  return [
    { kind: 'started',   run_id: runId },
    { kind: 'activity',  run_id: runId, activity_key: 'drafter' },
    { kind: 'activity',  run_id: runId, activity_key: 'verifier' },
    { kind: 'completed', run_id: runId, output: { verdict: 'approved' } },
  ]
}

describe('xlang verifier scenario -- TypeScript', () => {
  const events = makeXlangVerifierEvents(XLANG_VERIFIER_RUN_ID)

  it('started event is first', () => {
    expect(events[0].kind).toBe('started')
  })

  it('completed event is last', () => {
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('drafter activity comes before verifier', () => {
    const keys = events.filter(e => e.kind === 'activity').map(e => e.activity_key)
    expect(keys).toEqual(['drafter', 'verifier'])
  })

  it('completed output has verdict approved', () => {
    const last = events[events.length - 1]
    expect(last.output?.verdict).toBe('approved')
  })

  it('run_id consistent across events', () => {
    for (const ev of events) {
      expect(ev.run_id).toBe(XLANG_VERIFIER_RUN_ID)
    }
  })
})
