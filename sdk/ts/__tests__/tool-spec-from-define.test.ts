import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'
import { AgentSpecSchema } from '../schemas'

describe('ToolSpec from defineTool integrates with AgentSpecSchema', () => {
  it('spec from defineTool is valid for AgentSpec.tools', () => {
    const tool = defineTool({
      name: 'search',
      description: 'Search',
      schema: z.object({ query: z.string() }),
      handler: () => null,
    })
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4', tools: [tool.spec] })
    expect(result.success).toBe(true)
  })

  it('registry.specs array is valid for AgentSpec.tools', () => {
    const registry = new ToolRegistry()
    registry.register(defineTool({
      name: 'a',
      description: 'A',
      schema: z.object({ x: z.string() }),
      handler: () => null,
    }))
    registry.register(defineTool({
      name: 'b',
      description: 'B',
      schema: z.object({ y: z.number() }),
      handler: () => null,
    }))
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4', tools: registry.specs })
    expect(result.success).toBe(true)
    if (result.success) expect(result.data.tools).toHaveLength(2)
  })

  it('spec has correct required fields', () => {
    const tool = defineTool({
      name: 'greet',
      description: 'Greet',
      schema: z.object({ name: z.string(), title: z.string().optional() }),
      handler: () => null,
    })
    expect(tool.spec.input_schema.required).toContain('name')
    expect(tool.spec.input_schema.required).not.toContain('title')
  })
})
