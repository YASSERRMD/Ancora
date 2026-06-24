import { ToolCallSchema } from '../schemas'

describe('ToolCallSchema', () => {
  it('validates a tool call with string input', () => {
    const result = ToolCallSchema.safeParse({ name: 'search', input: { query: 'hello' } })
    expect(result.success).toBe(true)
  })

  it('validates a tool call with nested input', () => {
    const result = ToolCallSchema.safeParse({
      name: 'fetch',
      input: { url: 'https://example.com', options: { timeout: 5000 } },
    })
    expect(result.success).toBe(true)
  })

  it('validates empty input object', () => {
    expect(ToolCallSchema.safeParse({ name: 'noop', input: {} }).success).toBe(true)
  })

  it('rejects missing name', () => {
    expect(ToolCallSchema.safeParse({ input: {} }).success).toBe(false)
  })

  it('rejects empty name', () => {
    expect(ToolCallSchema.safeParse({ name: '', input: {} }).success).toBe(false)
  })

  it('rejects missing input', () => {
    expect(ToolCallSchema.safeParse({ name: 'search' }).success).toBe(false)
  })

  it('rejects non-object input', () => {
    expect(ToolCallSchema.safeParse({ name: 'search', input: 'not-an-object' }).success).toBe(false)
  })
})
