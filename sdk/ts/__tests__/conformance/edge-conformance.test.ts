import { parseEvent, encodeSpec, decodeSpec, validateSpec } from '../../wire'
import { AgentSpecSchema } from '../../schemas'
import { zodToInputSchema } from '../../zod-to-schema'
import { z } from 'zod'

describe('parseEvent edge cases', () => {
  it('throws on empty string', () => {
    expect(() => parseEvent('')).toThrow()
  })

  it('throws on JSON that is not an object', () => {
    expect(() => parseEvent('"just-a-string"')).toThrow()
  })

  it('throws when kind is missing', () => {
    expect(() => parseEvent('{"run_id":"r1"}')).toThrow()
  })

  it('accepts tool_call with empty input string', () => {
    const ev = parseEvent('{"kind":"tool_call","run_id":"r1","name":"fn","input":"{}"}')
    expect(ev.kind).toBe('tool_call')
  })

  it('preserves unicode in token text', () => {
    const ev = parseEvent('{"kind":"token","run_id":"r1","text":"日本語"}')
    if (ev.kind === 'token') expect(ev.text).toBe('日本語')
  })
})

describe('encodeSpec / decodeSpec edge cases', () => {
  it('spec with no optional fields roundtrips', () => {
    const spec = AgentSpecSchema.parse({ model: 'minimal' })
    const decoded = decodeSpec(encodeSpec(spec))
    expect(decoded.model).toBe('minimal')
  })

  it('spec with all optional fields roundtrips', () => {
    const spec = AgentSpecSchema.parse({
      model: 'full',
      instructions: 'Be helpful.',
      tools: [{ name: 'fn', description: 'd', input_schema: { type: 'object', properties: {}, required: [] } }],
      max_tokens: 512,
      temperature: 0.5,
    })
    const decoded = decodeSpec(encodeSpec(spec))
    expect(decoded.tools).toHaveLength(1)
    expect(decoded.max_tokens).toBe(512)
  })
})

describe('validateSpec edge cases', () => {
  it('errors list contains string messages', () => {
    const r = validateSpec({ temperature: 999 })
    if (!r.ok) {
      r.errors.forEach(e => expect(typeof e).toBe('string'))
    }
  })

  it('returns ok:false for non-object input', () => {
    expect(validateSpec(null).ok).toBe(false)
    expect(validateSpec(42).ok).toBe(false)
    expect(validateSpec('str').ok).toBe(false)
  })
})

describe('zodToInputSchema edge cases', () => {
  it('handles empty object schema', () => {
    const schema = zodToInputSchema(z.object({}))
    expect(schema.type).toBe('object')
    expect(schema.required).toEqual([])
  })

  it('handles nested optional fields', () => {
    const schema = zodToInputSchema(z.object({
      a: z.string(),
      b: z.string().optional(),
    }))
    expect(schema.required).toContain('a')
    expect(schema.required).not.toContain('b')
  })
})
