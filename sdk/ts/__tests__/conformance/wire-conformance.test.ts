import { encodeSpec, decodeSpec, parseEvent, validateSpec, buildSpec } from '../../wire'

const FULL_SPEC = {
  model: 'claude-3-5-sonnet',
  instructions: 'You are helpful.',
  tools: [],
  max_tokens: 1024,
  temperature: 0.7,
}

describe('wire protocol: encodeSpec / decodeSpec roundtrip', () => {
  it('roundtrips a full spec', () => {
    const buf = encodeSpec(FULL_SPEC)
    const decoded = decodeSpec(buf)
    expect(decoded.model).toBe(FULL_SPEC.model)
    expect(decoded.instructions).toBe(FULL_SPEC.instructions)
    expect(decoded.max_tokens).toBe(FULL_SPEC.max_tokens)
    expect(decoded.temperature).toBe(FULL_SPEC.temperature)
  })

  it('roundtrips a minimal spec', () => {
    const spec = buildSpec('my-model')
    const buf = encodeSpec(spec)
    expect(decodeSpec(buf).model).toBe('my-model')
  })

  it('encodeSpec returns a Buffer', () => {
    expect(Buffer.isBuffer(encodeSpec(FULL_SPEC))).toBe(true)
  })

  it('decodeSpec accepts Uint8Array', () => {
    const buf = encodeSpec(FULL_SPEC)
    const u8 = new Uint8Array(buf)
    const decoded = decodeSpec(u8)
    expect(decoded.model).toBe(FULL_SPEC.model)
  })
})

describe('wire protocol: parseEvent conformance', () => {
  it('parses started event', () => {
    const ev = parseEvent('{"kind":"started","run_id":"r1","spec":"{}"}')
    expect(ev.kind).toBe('started')
  })

  it('parses token event', () => {
    const ev = parseEvent('{"kind":"token","run_id":"r1","text":"hi"}')
    expect(ev.kind).toBe('token')
    if (ev.kind === 'token') expect(ev.text).toBe('hi')
  })

  it('parses completed event', () => {
    const ev = parseEvent('{"kind":"completed","run_id":"r1"}')
    expect(ev.kind).toBe('completed')
  })

  it('parses resumed event', () => {
    const ev = parseEvent('{"kind":"resumed","run_id":"r1","decision":"{}"}')
    expect(ev.kind).toBe('resumed')
  })

  it('parses tool_call event', () => {
    const ev = parseEvent('{"kind":"tool_call","run_id":"r1","name":"fn","input":"{}"}')
    expect(ev.kind).toBe('tool_call')
    if (ev.kind === 'tool_call') expect(ev.name).toBe('fn')
  })

  it('parses from Buffer', () => {
    const buf = Buffer.from('{"kind":"completed","run_id":"r1"}', 'utf8')
    expect(parseEvent(buf).kind).toBe('completed')
  })

  it('parses from Uint8Array', () => {
    const bytes = new TextEncoder().encode('{"kind":"completed","run_id":"r1"}')
    expect(parseEvent(bytes).kind).toBe('completed')
  })

  it('throws on invalid JSON', () => {
    expect(() => parseEvent('not-json')).toThrow()
  })

  it('throws on unknown event kind', () => {
    expect(() => parseEvent('{"kind":"mystery","run_id":"r1"}')).toThrow()
  })
})

describe('wire protocol: validateSpec and buildSpec', () => {
  it('validateSpec returns ok for a valid spec', () => {
    const r = validateSpec(FULL_SPEC)
    expect(r.ok).toBe(true)
  })

  it('validateSpec returns errors for an invalid spec', () => {
    const r = validateSpec({ max_tokens: 512 })
    expect(r.ok).toBe(false)
    if (!r.ok) expect(r.errors.length).toBeGreaterThan(0)
  })

  it('buildSpec creates a spec with correct model', () => {
    const s = buildSpec('gpt-4', { maxTokens: 500 })
    expect(s.model).toBe('gpt-4')
    expect(s.max_tokens).toBe(500)
  })

  it('buildSpec sets sensible defaults', () => {
    const s = buildSpec('gpt-4')
    expect(typeof s.model).toBe('string')
  })
})
