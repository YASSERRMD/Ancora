import { AgentSpecSchema } from '../../schemas'
import { encodeSpec, decodeSpec } from '../../wire'

describe('AgentSpec field serialization conformance', () => {
  it('max_tokens default is undefined when not provided', () => {
    const spec = AgentSpecSchema.parse({ model: 'x' })
    expect(spec.max_tokens).toBeUndefined()
  })

  it('temperature default is undefined when not provided', () => {
    const spec = AgentSpecSchema.parse({ model: 'x' })
    expect(spec.temperature).toBeUndefined()
  })

  it('tools default to empty array', () => {
    const spec = AgentSpecSchema.parse({ model: 'x' })
    expect(spec.tools).toEqual([])
  })

  it('instructions default is empty string when not provided', () => {
    const spec = AgentSpecSchema.parse({ model: 'x' })
    expect(spec.instructions).toBe('')
  })

  it('spec with temperature 0.0 roundtrips correctly', () => {
    const spec = AgentSpecSchema.parse({ model: 'x', temperature: 0.0 })
    expect(decodeSpec(encodeSpec(spec)).temperature).toBe(0.0)
  })

  it('spec with temperature 1.0 roundtrips correctly', () => {
    const spec = AgentSpecSchema.parse({ model: 'x', temperature: 1.0 })
    expect(decodeSpec(encodeSpec(spec)).temperature).toBe(1.0)
  })

  it('spec with max_tokens 1 roundtrips correctly', () => {
    const spec = AgentSpecSchema.parse({ model: 'x', max_tokens: 1 })
    expect(decodeSpec(encodeSpec(spec)).max_tokens).toBe(1)
  })

  it('tool spec has all three required fields', () => {
    const spec = AgentSpecSchema.parse({
      model: 'x',
      tools: [{ name: 'fn', description: 'a function', input_schema: { type: 'object', properties: {}, required: [] } }],
    })
    expect(spec.tools[0].name).toBe('fn')
    expect(spec.tools[0].description).toBe('a function')
    expect(spec.tools[0].input_schema.type).toBe('object')
  })

  it('tool spec roundtrips via encode/decode', () => {
    const spec = AgentSpecSchema.parse({
      model: 'x',
      tools: [{ name: 'greet', description: 'say hi', input_schema: { type: 'object', properties: { name: { type: 'string' } }, required: ['name'] } }],
    })
    const decoded = decodeSpec(encodeSpec(spec))
    expect(decoded.tools[0].name).toBe('greet')
    expect(decoded.tools[0].input_schema.required).toContain('name')
  })
})
