import { z } from 'zod'
import { defineTool } from '../tools'

describe('defineTool', () => {
  it('returns a ToolDef with correct spec name', () => {
    const tool = defineTool({
      name: 'search',
      description: 'Search the web',
      schema: z.object({ query: z.string() }),
      handler: ({ query }) => `results for ${query}`,
    })
    expect(tool.spec.name).toBe('search')
  })

  it('sets description on spec', () => {
    const tool = defineTool({
      name: 'noop',
      description: 'Does nothing',
      schema: z.object({}),
      handler: () => null,
    })
    expect(tool.spec.description).toBe('Does nothing')
  })

  it('infers input_schema from zod schema', () => {
    const tool = defineTool({
      name: 'calc',
      description: 'Calc',
      schema: z.object({ a: z.number(), b: z.number() }),
      handler: ({ a, b }) => a + b,
    })
    expect(tool.spec.input_schema.properties.a.type).toBe('number')
    expect(tool.spec.input_schema.required).toContain('a')
    expect(tool.spec.input_schema.required).toContain('b')
  })

  it('handler can be called with valid input', async () => {
    const tool = defineTool({
      name: 'greet',
      description: 'Say hi',
      schema: z.object({ name: z.string() }),
      handler: ({ name }) => `Hello, ${name}!`,
    })
    const result = await Promise.resolve(tool.handler({ name: 'Alice' }))
    expect(result).toBe('Hello, Alice!')
  })

  it('async handler is supported', async () => {
    const tool = defineTool({
      name: 'fetch',
      description: 'Async tool',
      schema: z.object({ url: z.string() }),
      handler: async ({ url }) => `fetched: ${url}`,
    })
    const result = await tool.handler({ url: 'https://example.com' })
    expect(result).toBe('fetched: https://example.com')
  })
})
