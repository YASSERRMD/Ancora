import { AgentSpecSchema, ToolSpecSchema } from '../schemas'
import { encodeSpec, decodeSpec } from '../wire'

const SEARCH_TOOL = {
  name: 'search',
  description: 'Search the web',
  input_schema: {
    type: 'object' as const,
    properties: {
      query: { type: 'string', description: 'Search query' },
      limit: { type: 'number', description: 'Max results' },
    },
    required: ['query'],
  },
}

const FETCH_TOOL = {
  name: 'fetch',
  description: 'Fetch a URL',
  input_schema: {
    type: 'object' as const,
    properties: { url: { type: 'string' } },
    required: ['url'],
  },
}

describe('multi-tool AgentSpec', () => {
  it('ToolSpec validates a tool with multiple properties', () => {
    expect(ToolSpecSchema.safeParse(SEARCH_TOOL).success).toBe(true)
  })

  it('AgentSpec accepts multiple tools', () => {
    const spec = { model: 'gpt-4', tools: [SEARCH_TOOL, FETCH_TOOL] }
    const result = AgentSpecSchema.safeParse(spec)
    expect(result.success).toBe(true)
    if (result.success) expect(result.data.tools).toHaveLength(2)
  })

  it('multi-tool spec survives round-trip', () => {
    const result = AgentSpecSchema.safeParse({ model: 'gpt-4', tools: [SEARCH_TOOL, FETCH_TOOL] })
    expect(result.success).toBe(true)
    const spec = (result as { success: true; data: ReturnType<typeof AgentSpecSchema.parse> }).data
    const decoded = decodeSpec(encodeSpec(spec))
    expect(decoded.tools).toHaveLength(2)
    expect(decoded.tools[0].name).toBe('search')
    expect(decoded.tools[1].name).toBe('fetch')
  })

  it('tool properties are preserved round-trip', () => {
    const spec = AgentSpecSchema.parse({ model: 'gpt-4', tools: [SEARCH_TOOL] })
    const decoded = decodeSpec(encodeSpec(spec))
    const props = decoded.tools[0].input_schema.properties
    expect(Object.keys(props)).toContain('query')
    expect(Object.keys(props)).toContain('limit')
  })

  it('required array is preserved round-trip', () => {
    const spec = AgentSpecSchema.parse({ model: 'gpt-4', tools: [SEARCH_TOOL] })
    const decoded = decodeSpec(encodeSpec(spec))
    expect(decoded.tools[0].input_schema.required).toEqual(['query'])
  })
})
