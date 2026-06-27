import { AgentSpecSchema } from '../schemas'
import { z } from 'zod'
import { zodToInputSchema } from '../zod-to-schema'

describe('phase144 spec round-trip', () => {
  it('parses minimal spec', () => {
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4o' })
    expect(result.success).toBe(true)
  })

  it('model field round-trips', () => {
    const result = AgentSpecSchema.safeParse({ model: 'claude-opus-4-8' })
    expect(result.success && result.data.model).toBe('claude-opus-4-8')
  })

  it('instructions default to empty string', () => {
    const result = AgentSpecSchema.safeParse({ model: 'llama3' })
    expect(result.success && result.data.instructions).toBe('')
  })

  it('tools default to empty array', () => {
    const result = AgentSpecSchema.safeParse({ model: 'llama3' })
    expect(result.success && (result.data as { tools: unknown[] }).tools).toHaveLength(0)
  })

  it('accepts tools array', () => {
    const tool = {
      name: 'search',
      description: 'Search',
      input_schema: { type: 'object' as const, properties: {}, required: [] },
    }
    const result = AgentSpecSchema.safeParse({ model: 'llama3', tools: [tool] })
    expect(result.success).toBe(true)
  })

  it('rejects missing model', () => {
    expect(AgentSpecSchema.safeParse({ instructions: 'hi' }).success).toBe(false)
  })

  it('rejects empty model string', () => {
    expect(AgentSpecSchema.safeParse({ model: '' }).success).toBe(false)
  })

  it('zod schema converts to input schema', () => {
    const s = z.object({ query: z.string(), limit: z.number() })
    const schema = zodToInputSchema(s)
    expect(schema.properties.query.type).toBe('string')
    expect(schema.properties.limit.type).toBe('number')
  })

  it('spec with max_tokens round-trips', () => {
    const result = AgentSpecSchema.safeParse({ model: 'llama3', max_tokens: 512 })
    expect(result.success && result.data.max_tokens).toBe(512)
  })

  it('rejects negative max_tokens', () => {
    expect(AgentSpecSchema.safeParse({ model: 'llama3', max_tokens: -1 }).success).toBe(false)
  })
})
