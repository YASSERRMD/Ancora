import { z } from 'zod'
import { zodToInputSchema } from '../zod-to-schema'

describe('zodToInputSchema', () => {
  it('converts a string field', () => {
    const schema = z.object({ query: z.string() })
    const result = zodToInputSchema(schema)
    expect(result.properties.query.type).toBe('string')
  })

  it('converts a number field', () => {
    const schema = z.object({ limit: z.number() })
    const result = zodToInputSchema(schema)
    expect(result.properties.limit.type).toBe('number')
  })

  it('converts a boolean field', () => {
    const schema = z.object({ flag: z.boolean() })
    const result = zodToInputSchema(schema)
    expect(result.properties.flag.type).toBe('boolean')
  })

  it('marks required fields', () => {
    const schema = z.object({ name: z.string(), age: z.number() })
    const result = zodToInputSchema(schema)
    expect(result.required).toContain('name')
    expect(result.required).toContain('age')
  })

  it('optional fields are not required', () => {
    const schema = z.object({ name: z.string(), note: z.string().optional() })
    const result = zodToInputSchema(schema)
    expect(result.required).toContain('name')
    expect(result.required).not.toContain('note')
  })

  it('preserves field descriptions', () => {
    const schema = z.object({ query: z.string().describe('The search query') })
    const result = zodToInputSchema(schema)
    expect(result.properties.query.description).toBe('The search query')
  })

  it('output has type=object', () => {
    const schema = z.object({ x: z.string() })
    expect(zodToInputSchema(schema).type).toBe('object')
  })

  it('empty object schema produces empty properties', () => {
    const schema = z.object({})
    const result = zodToInputSchema(schema)
    expect(Object.keys(result.properties)).toHaveLength(0)
    expect(result.required).toHaveLength(0)
  })
})
