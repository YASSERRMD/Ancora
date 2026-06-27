import { z } from 'zod'
import { zodToInputSchema } from '../zod-to-schema'
import { AgentSpecSchema } from '../schemas'

const AnswerSchema = z.object({
  answer: z.string(),
  confidence: z.number().min(0).max(1),
  sources: z.array(z.string()),
})

describe('phase144 structured output zod', () => {
  it('zodToInputSchema converts AnswerSchema correctly', () => {
    const schema = zodToInputSchema(AnswerSchema)
    expect(schema.type).toBe('object')
    expect(schema.properties.answer.type).toBe('string')
    expect(schema.properties.confidence.type).toBe('number')
  })

  it('AnswerSchema validates correct shape', () => {
    const result = AnswerSchema.safeParse({
      answer: 'Paris',
      confidence: 0.95,
      sources: ['Wikipedia'],
    })
    expect(result.success).toBe(true)
  })

  it('AnswerSchema rejects missing answer', () => {
    const result = AnswerSchema.safeParse({ confidence: 0.5, sources: [] })
    expect(result.success).toBe(false)
  })

  it('AnswerSchema rejects confidence above 1', () => {
    const result = AnswerSchema.safeParse({ answer: 'x', confidence: 1.1, sources: [] })
    expect(result.success).toBe(false)
  })

  it('AnswerSchema rejects confidence below 0', () => {
    const result = AnswerSchema.safeParse({ answer: 'x', confidence: -0.1, sources: [] })
    expect(result.success).toBe(false)
  })

  it('sources is an array of strings', () => {
    const result = AnswerSchema.safeParse({ answer: 'x', confidence: 0.5, sources: ['a', 'b'] })
    expect(result.success && result.data.sources).toHaveLength(2)
  })

  it('empty sources array is valid', () => {
    const result = AnswerSchema.safeParse({ answer: 'y', confidence: 0.0, sources: [] })
    expect(result.success).toBe(true)
  })

  it('zodToInputSchema includes required fields', () => {
    const schema = zodToInputSchema(z.object({ name: z.string(), age: z.number() }))
    expect(schema.required).toContain('name')
    expect(schema.required).toContain('age')
  })

  it('optional fields not required', () => {
    const s = z.object({ name: z.string(), nick: z.string().optional() })
    const schema = zodToInputSchema(s)
    expect(schema.required).toContain('name')
    expect(schema.required ?? []).not.toContain('nick')
  })

  it('AgentSpecSchema accepts model with instructions', () => {
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4o', instructions: 'Reply in JSON' })
    expect(result.success).toBe(true)
  })
})
