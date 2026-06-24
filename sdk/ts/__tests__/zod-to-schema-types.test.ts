import { z } from 'zod'
import { zodToInputSchema } from '../zod-to-schema'

describe('zodToInputSchema type coverage', () => {
  it('handles array type', () => {
    const schema = z.object({ items: z.array(z.string()) })
    const result = zodToInputSchema(schema)
    expect(result.properties.items.type).toBe('array')
  })

  it('handles nested object type', () => {
    const schema = z.object({ nested: z.object({ x: z.string() }) })
    const result = zodToInputSchema(schema)
    expect(result.properties.nested.type).toBe('object')
  })

  it('handles nullable fields as optional', () => {
    const schema = z.object({ maybe: z.string().nullable() })
    const result = zodToInputSchema(schema)
    expect(result.required).not.toContain('maybe')
  })

  it('handles mixed required and optional fields', () => {
    const schema = z.object({
      req1: z.string(),
      req2: z.number(),
      opt1: z.boolean().optional(),
      opt2: z.string().nullable(),
    })
    const result = zodToInputSchema(schema)
    expect(result.required).toContain('req1')
    expect(result.required).toContain('req2')
    expect(result.required).not.toContain('opt1')
    expect(result.required).not.toContain('opt2')
    expect(Object.keys(result.properties)).toHaveLength(4)
  })
})
