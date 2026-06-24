import { AgentSpecSchema } from '../schemas'

const TOOL = {
  name: 'search',
  description: 'Search',
  input_schema: { type: 'object' as const, properties: {}, required: [] },
}

describe('AgentSpecSchema', () => {
  it('validates a minimal spec', () => {
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4' })
    expect(result.success).toBe(true)
  })

  it('defaults instructions to empty string', () => {
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4' })
    expect(result.success).toBe(true)
    if (result.success) {
      expect(result.data.instructions).toBe('')
    }
  })

  it('defaults tools to empty array', () => {
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4' })
    expect(result.success).toBe(true)
    if (result.success) {
      expect(result.data.tools).toEqual([])
    }
  })

  it('validates a full spec with tools', () => {
    const spec = {
      model: 'gpt-4',
      instructions: 'Be helpful',
      tools: [TOOL],
      max_tokens: 1024,
      temperature: 0.7,
    }
    const result = AgentSpecSchema.safeParse(spec)
    expect(result.success).toBe(true)
  })

  it('rejects missing model', () => {
    expect(AgentSpecSchema.safeParse({ instructions: 'hi' }).success).toBe(false)
  })

  it('rejects empty model string', () => {
    expect(AgentSpecSchema.safeParse({ model: '' }).success).toBe(false)
  })

  it('rejects negative max_tokens', () => {
    expect(AgentSpecSchema.safeParse({ model: 'gpt-4', max_tokens: -1 }).success).toBe(false)
  })

  it('rejects temperature above 2', () => {
    expect(AgentSpecSchema.safeParse({ model: 'gpt-4', temperature: 2.5 }).success).toBe(false)
  })

  it('rejects temperature below 0', () => {
    expect(AgentSpecSchema.safeParse({ model: 'gpt-4', temperature: -0.1 }).success).toBe(false)
  })

  it('accepts temperature at boundary values 0 and 2', () => {
    expect(AgentSpecSchema.safeParse({ model: 'gpt-4', temperature: 0 }).success).toBe(true)
    expect(AgentSpecSchema.safeParse({ model: 'gpt-4', temperature: 2 }).success).toBe(true)
  })
})
