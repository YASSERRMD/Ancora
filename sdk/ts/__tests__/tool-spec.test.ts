import { ToolSpecSchema } from '../schemas'

const VALID_TOOL = {
  name: 'search',
  description: 'Search the web',
  input_schema: {
    type: 'object' as const,
    properties: { query: { type: 'string', description: 'Search query' } },
    required: ['query'],
  },
}

describe('ToolSpecSchema', () => {
  it('validates a valid tool', () => {
    const result = ToolSpecSchema.safeParse(VALID_TOOL)
    expect(result.success).toBe(true)
  })

  it('rejects missing name', () => {
    const { name: _n, ...noName } = VALID_TOOL
    expect(ToolSpecSchema.safeParse(noName).success).toBe(false)
  })

  it('rejects empty name', () => {
    const result = ToolSpecSchema.safeParse({ ...VALID_TOOL, name: '' })
    expect(result.success).toBe(false)
  })

  it('rejects missing description', () => {
    const { description: _d, ...noDesc } = VALID_TOOL
    expect(ToolSpecSchema.safeParse(noDesc).success).toBe(false)
  })

  it('rejects missing input_schema', () => {
    const { input_schema: _s, ...noSchema } = VALID_TOOL
    expect(ToolSpecSchema.safeParse(noSchema).success).toBe(false)
  })

  it('defaults properties to {} when omitted', () => {
    const tool = { ...VALID_TOOL, input_schema: { type: 'object' as const } }
    const result = ToolSpecSchema.safeParse(tool)
    expect(result.success).toBe(true)
    if (result.success) {
      expect(result.data.input_schema.properties).toEqual({})
    }
  })

  it('defaults required to [] when omitted', () => {
    const tool = { ...VALID_TOOL, input_schema: { type: 'object' as const } }
    const result = ToolSpecSchema.safeParse(tool)
    expect(result.success).toBe(true)
    if (result.success) {
      expect(result.data.input_schema.required).toEqual([])
    }
  })
})
