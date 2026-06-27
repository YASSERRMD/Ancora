// Cross-language A2A: TypeScript hands off to .NET over A2A (offline fixture).

interface A2AEnvelope {
  protocol: string
  sender: { lang: string; sdk_version?: string }
  recipient: { lang: string; sdk_version?: string }
  run_id: string
  payload: Record<string, unknown>
}

const HANDOFF_RUN_ID = 'a2a-ts-dotnet-001'

const A2A_ENVELOPE: A2AEnvelope = {
  protocol: 'a2a/1.0',
  sender: { lang: 'ts', sdk_version: '0.3.0' },
  recipient: { lang: 'dotnet', sdk_version: '0.3.0' },
  run_id: HANDOFF_RUN_ID,
  payload: {
    task: 'process',
    data: 'TypeScript produced this data',
    handoff_reason: 'processor runs in .NET',
  },
}

const A2A_RESPONSE: A2AEnvelope = {
  protocol: 'a2a/1.0',
  sender: { lang: 'dotnet' },
  recipient: { lang: 'ts' },
  run_id: HANDOFF_RUN_ID,
  payload: { status: 'processed', result: 'dotnet-ok' },
}

describe('xlang A2A: TypeScript -> .NET', () => {
  it('envelope protocol is a2a/1.0', () => {
    expect(A2A_ENVELOPE.protocol).toBe('a2a/1.0')
  })

  it('sender is ts', () => {
    expect(A2A_ENVELOPE.sender.lang).toBe('ts')
  })

  it('recipient is dotnet', () => {
    expect(A2A_ENVELOPE.recipient.lang).toBe('dotnet')
  })

  it('response run_id matches envelope run_id', () => {
    expect(A2A_RESPONSE.run_id).toBe(A2A_ENVELOPE.run_id)
  })

  it('response sender is dotnet', () => {
    expect(A2A_RESPONSE.sender.lang).toBe('dotnet')
  })

  it('response payload has status', () => {
    expect(A2A_RESPONSE.payload.status).toBe('processed')
  })

  it('envelope serialises to valid JSON', () => {
    const raw = JSON.stringify(A2A_ENVELOPE)
    const decoded: A2AEnvelope = JSON.parse(raw)
    expect(decoded.protocol).toBe('a2a/1.0')
  })

  it('payload has handoff_reason', () => {
    expect(A2A_ENVELOPE.payload.handoff_reason).toBeTruthy()
  })
})
