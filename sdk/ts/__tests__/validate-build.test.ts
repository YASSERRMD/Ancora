import { validateSpec, buildSpec } from '../wire'

describe('validateSpec', () => {
  it('returns ok=true for valid spec', () => {
    const result = validateSpec({ model: 'gpt-4' })
    expect(result.ok).toBe(true)
    if (result.ok) expect(result.spec.model).toBe('gpt-4')
  })

  it('returns ok=false for missing model', () => {
    const result = validateSpec({ instructions: 'hi' })
    expect(result.ok).toBe(false)
    if (!result.ok) expect(result.errors.length).toBeGreaterThan(0)
  })

  it('error messages reference the failing field', () => {
    const result = validateSpec({ model: '' })
    expect(result.ok).toBe(false)
    if (!result.ok) {
      const combined = result.errors.join(' ')
      expect(combined).toContain('model')
    }
  })

  it('returns ok=true for spec with tools', () => {
    const result = validateSpec({
      model: 'gpt-4',
      tools: [{ name: 'search', description: 'Search', input_schema: { type: 'object' } }],
    })
    expect(result.ok).toBe(true)
  })
})

describe('buildSpec', () => {
  it('builds a minimal spec with just model', () => {
    const spec = buildSpec('gpt-4')
    expect(spec.model).toBe('gpt-4')
    expect(spec.instructions).toBe('')
    expect(spec.tools).toEqual([])
  })

  it('builds a spec with options', () => {
    const spec = buildSpec('gpt-4', { instructions: 'Be concise', maxTokens: 512 })
    expect(spec.instructions).toBe('Be concise')
    expect(spec.max_tokens).toBe(512)
  })

  it('throws on invalid model', () => {
    expect(() => buildSpec('')).toThrow()
  })

  it('throws on invalid temperature', () => {
    expect(() => buildSpec('gpt-4', { temperature: 5 })).toThrow()
  })
})
