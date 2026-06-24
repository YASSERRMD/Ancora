import { z } from 'zod'
import { zodToInputSchema } from '../../zod-to-schema'

describe('zodToInputSchema conformance for all types', () => {
  it('maps string to type string', () => {
    const s = zodToInputSchema(z.object({ x: z.string() }))
    expect(s.properties['x'].type).toBe('string')
  })

  it('maps number to type number', () => {
    const s = zodToInputSchema(z.object({ x: z.number() }))
    expect(s.properties['x'].type).toBe('number')
  })

  it('maps boolean to type boolean', () => {
    const s = zodToInputSchema(z.object({ x: z.boolean() }))
    expect(s.properties['x'].type).toBe('boolean')
  })

  it('maps z.array(z.string()) to type array', () => {
    const s = zodToInputSchema(z.object({ x: z.array(z.string()) }))
    expect(s.properties['x'].type).toBe('array')
  })

  it('maps nested z.object to type object', () => {
    const s = zodToInputSchema(z.object({ x: z.object({ y: z.string() }) }))
    expect(s.properties['x'].type).toBe('object')
  })

  it('required contains only non-optional fields', () => {
    const s = zodToInputSchema(z.object({
      a: z.string(),
      b: z.number().optional(),
      c: z.boolean(),
    }))
    expect(s.required).toContain('a')
    expect(s.required).toContain('c')
    expect(s.required).not.toContain('b')
  })

  it('nullable field is not required', () => {
    const s = zodToInputSchema(z.object({ x: z.string().nullable() }))
    expect(s.required).not.toContain('x')
  })

  it('description is forwarded from .describe()', () => {
    const s = zodToInputSchema(z.object({ x: z.string().describe('the value') }))
    expect(s.properties['x'].description).toBe('the value')
  })

  it('schema type is always object', () => {
    const s = zodToInputSchema(z.object({}))
    expect(s.type).toBe('object')
  })

  it('multiple fields all appear in properties', () => {
    const s = zodToInputSchema(z.object({ a: z.string(), b: z.number(), c: z.boolean() }))
    expect(Object.keys(s.properties).sort()).toEqual(['a', 'b', 'c'])
  })
})
