import { z } from 'zod'
import { zodToInputSchema, buildSpec, defineTool } from '../../index'

const searchSchema = z.object({
  query: z.string().describe('The search query'),
  limit: z.number().optional().describe('Maximum results'),
  exact: z.boolean().optional(),
})

describe('schema-gen example smoke test', () => {
  it('zodToInputSchema produces valid JSON Schema', () => {
    const schema = zodToInputSchema(searchSchema)
    expect(schema.type).toBe('object')
    expect(schema.properties['query'].type).toBe('string')
    expect(schema.properties['query'].description).toBe('The search query')
    expect(schema.properties['limit'].type).toBe('number')
    expect(schema.required).toContain('query')
    expect(schema.required).not.toContain('limit')
  })

  it('defineTool spec matches zodToInputSchema output', () => {
    const tool = defineTool({
      name: 'search',
      description: 'Search a knowledge base',
      schema: searchSchema,
      handler: async ({ query }) => ({ results: [query] }),
    })
    expect(tool.spec.name).toBe('search')
    expect(tool.spec.input_schema.required).toContain('query')
    expect(Object.keys(tool.spec.input_schema.properties)).toContain('query')
  })

  it('buildSpec includes tool in tools array', () => {
    const tool = defineTool({
      name: 'search',
      description: 'Search',
      schema: searchSchema,
      handler: async ({ query }) => [query],
    })
    const spec = buildSpec('claude', { instructions: 'Use tools.', tools: [tool.spec] })
    expect(spec.tools).toHaveLength(1)
    expect(spec.tools[0].name).toBe('search')
  })
})
