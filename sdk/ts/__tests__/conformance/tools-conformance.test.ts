import { z } from 'zod'
import { defineTool, ToolRegistry } from '../../tools'
import { zodToInputSchema } from '../../zod-to-schema'

const greetTool = defineTool({
  name: 'greet',
  description: 'Greet someone by name',
  schema: z.object({ name: z.string(), loud: z.boolean().optional() }),
  handler: ({ name, loud }) => (loud ? name.toUpperCase() : name),
})

describe('zodToInputSchema conformance', () => {
  it('maps string fields', () => {
    const schema = zodToInputSchema(z.object({ q: z.string() }))
    expect(schema.properties['q'].type).toBe('string')
  })

  it('maps number fields', () => {
    const schema = zodToInputSchema(z.object({ n: z.number() }))
    expect(schema.properties['n'].type).toBe('number')
  })

  it('maps boolean fields', () => {
    const schema = zodToInputSchema(z.object({ flag: z.boolean() }))
    expect(schema.properties['flag'].type).toBe('boolean')
  })

  it('marks required fields', () => {
    const schema = zodToInputSchema(z.object({ req: z.string(), opt: z.string().optional() }))
    expect(schema.required).toContain('req')
    expect(schema.required).not.toContain('opt')
  })

  it('preserves .describe() descriptions', () => {
    const schema = zodToInputSchema(z.object({ city: z.string().describe('The city name') }))
    expect(schema.properties['city'].description).toBe('The city name')
  })
})

describe('defineTool conformance', () => {
  it('creates a ToolDef with correct spec name', () => {
    expect(greetTool.spec.name).toBe('greet')
  })

  it('creates a ToolDef with correct description', () => {
    expect(greetTool.spec.description).toBe('Greet someone by name')
  })

  it('handler returns expected result', async () => {
    const result = await greetTool.handler({ name: 'Alice', loud: false })
    expect(result).toBe('Alice')
  })

  it('handler works with optional fields', async () => {
    const result = await greetTool.handler({ name: 'Bob', loud: true })
    expect(result).toBe('BOB')
  })

  it('spec.input_schema has type object', () => {
    expect(greetTool.spec.input_schema.type).toBe('object')
  })
})

describe('ToolRegistry conformance', () => {
  it('register adds a tool and has() returns true', () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    expect(reg.has('greet')).toBe(true)
  })

  it('has() returns false for unregistered tool', () => {
    const reg = new ToolRegistry()
    expect(reg.has('unknown')).toBe(false)
  })

  it('names includes registered tool names', () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    expect(reg.names).toContain('greet')
  })

  it('specs includes registered tool specs', () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    expect(reg.specs.map(s => s.name)).toContain('greet')
  })

  it('dispatch calls the handler with parsed input', async () => {
    const reg = new ToolRegistry()
    reg.register(greetTool)
    const result = await reg.dispatch('greet', { name: 'Carol', loud: false })
    expect(result).toBe('Carol')
  })

  it('dispatch throws for unknown tool', async () => {
    const reg = new ToolRegistry()
    await expect(reg.dispatch('nope', {})).rejects.toThrow()
  })
})
