import { encodeSpec, decodeSpec, parseEvent } from '../wire'
import { AgentSpec } from '../schemas'

const TOOL = {
  name: 'search',
  description: 'Search',
  input_schema: { type: 'object' as const, properties: {}, required: [] },
}

const MINIMAL_SPEC: AgentSpec = {
  model: 'gpt-4',
  instructions: '',
  tools: [],
}

const FULL_SPEC: AgentSpec = {
  model: 'claude-3',
  instructions: 'Be concise',
  tools: [TOOL],
  max_tokens: 512,
  temperature: 0.5,
}

describe('encodeSpec', () => {
  it('returns a Buffer', () => {
    const buf = encodeSpec(MINIMAL_SPEC)
    expect(Buffer.isBuffer(buf)).toBe(true)
  })

  it('returns non-empty bytes', () => {
    const buf = encodeSpec(MINIMAL_SPEC)
    expect(buf.length).toBeGreaterThan(0)
  })

  it('encodes to valid JSON', () => {
    const buf = encodeSpec(FULL_SPEC)
    expect(() => JSON.parse(buf.toString('utf8'))).not.toThrow()
  })

  it('works with empty tools array', () => {
    const buf = encodeSpec({ model: 'gpt-4', instructions: '', tools: [] })
    const parsed = JSON.parse(buf.toString('utf8'))
    expect(parsed.tools).toEqual([])
  })
})

describe('decodeSpec', () => {
  it('returns a valid AgentSpec', () => {
    const buf = encodeSpec(FULL_SPEC)
    const decoded = decodeSpec(buf)
    expect(decoded.model).toBe('claude-3')
  })

  it('throws on invalid JSON', () => {
    expect(() => decodeSpec(Buffer.from('not json'))).toThrow()
  })

  it('throws on missing model', () => {
    const bad = Buffer.from(JSON.stringify({ instructions: 'hi' }), 'utf8')
    expect(() => decodeSpec(bad)).toThrow()
  })
})

describe('round-trip a spec', () => {
  it('minimal spec survives encode then decode', () => {
    const encoded = encodeSpec(MINIMAL_SPEC)
    const decoded = decodeSpec(encoded)
    expect(decoded.model).toBe(MINIMAL_SPEC.model)
    expect(decoded.instructions).toBe(MINIMAL_SPEC.instructions)
    expect(decoded.tools).toEqual(MINIMAL_SPEC.tools)
  })

  it('full spec survives encode then decode', () => {
    const encoded = encodeSpec(FULL_SPEC)
    const decoded = decodeSpec(encoded)
    expect(decoded.model).toBe(FULL_SPEC.model)
    expect(decoded.instructions).toBe(FULL_SPEC.instructions)
    expect(decoded.max_tokens).toBe(FULL_SPEC.max_tokens)
    expect(decoded.temperature).toBe(FULL_SPEC.temperature)
    expect(decoded.tools).toHaveLength(1)
    expect(decoded.tools[0].name).toBe('search')
  })
})

describe('parseEvent', () => {
  it('parses a started event', () => {
    const ev = parseEvent('{"kind":"started","run_id":"r1","spec":"{}"}')
    expect(ev.kind).toBe('started')
    if (ev.kind === 'started') expect(ev.run_id).toBe('r1')
  })

  it('parses a token event', () => {
    const ev = parseEvent('{"kind":"token","run_id":"r1","text":"Hello"}')
    expect(ev.kind).toBe('token')
    if (ev.kind === 'token') expect(ev.text).toBe('Hello')
  })

  it('parses a completed event', () => {
    const ev = parseEvent('{"kind":"completed","run_id":"r1"}')
    expect(ev.kind).toBe('completed')
  })

  it('parses a resumed event', () => {
    const ev = parseEvent('{"kind":"resumed","run_id":"r1","decision":"approve"}')
    expect(ev.kind).toBe('resumed')
    if (ev.kind === 'resumed') expect(ev.decision).toBe('approve')
  })

  it('throws on unknown event kind', () => {
    expect(() => parseEvent('{"kind":"unknown","run_id":"r1"}')).toThrow()
  })

  it('accepts Buffer input', () => {
    const buf = Buffer.from('{"kind":"completed","run_id":"r2"}', 'utf8')
    const ev = parseEvent(buf)
    expect(ev.kind).toBe('completed')
  })
})
