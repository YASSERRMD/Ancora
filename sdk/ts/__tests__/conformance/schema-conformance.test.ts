import { AgentSpecSchema, RunEventSchema, ToolSpecSchema, ToolCallSchema } from '../../schemas'

describe('AgentSpecSchema conformance', () => {
  it('accepts a minimal spec with only model', () => {
    const result = AgentSpecSchema.safeParse({ model: 'claude-3-5-sonnet' })
    expect(result.success).toBe(true)
  })

  it('accepts a full spec with all fields', () => {
    const result = AgentSpecSchema.safeParse({
      model: 'claude-3-5-sonnet',
      instructions: 'You are helpful.',
      tools: [],
      max_tokens: 1024,
      temperature: 0.7,
    })
    expect(result.success).toBe(true)
  })

  it('rejects a spec with missing model', () => {
    const result = AgentSpecSchema.safeParse({ instructions: 'hi' })
    expect(result.success).toBe(false)
  })

  it('rejects negative max_tokens', () => {
    const result = AgentSpecSchema.safeParse({ model: 'claude', max_tokens: -1 })
    expect(result.success).toBe(false)
  })

  it('rejects temperature out of range', () => {
    const result = AgentSpecSchema.safeParse({ model: 'claude', temperature: 2.5 })
    expect(result.success).toBe(false)
  })
})

describe('RunEventSchema conformance', () => {
  it('accepts started event', () => {
    const r = RunEventSchema.safeParse({ kind: 'started', run_id: 'r1', spec: '{}' })
    expect(r.success).toBe(true)
  })

  it('accepts token event', () => {
    const r = RunEventSchema.safeParse({ kind: 'token', run_id: 'r1', text: 'hi' })
    expect(r.success).toBe(true)
  })

  it('accepts completed event', () => {
    const r = RunEventSchema.safeParse({ kind: 'completed', run_id: 'r1' })
    expect(r.success).toBe(true)
  })

  it('accepts resumed event', () => {
    const r = RunEventSchema.safeParse({ kind: 'resumed', run_id: 'r1', decision: '{}' })
    expect(r.success).toBe(true)
  })

  it('accepts tool_call event', () => {
    const r = RunEventSchema.safeParse({ kind: 'tool_call', run_id: 'r1', name: 'fn', input: '{}' })
    expect(r.success).toBe(true)
  })

  it('rejects event with unknown kind', () => {
    const r = RunEventSchema.safeParse({ kind: 'unknown', run_id: 'r1' })
    expect(r.success).toBe(false)
  })

  it('rejects event missing run_id', () => {
    const r = RunEventSchema.safeParse({ kind: 'completed' })
    expect(r.success).toBe(false)
  })
})

describe('ToolSpecSchema and ToolCallSchema conformance', () => {
  it('accepts a valid tool spec', () => {
    const r = ToolSpecSchema.safeParse({
      name: 'search',
      description: 'Search the web',
      input_schema: { type: 'object', properties: {}, required: [] },
    })
    expect(r.success).toBe(true)
  })

  it('rejects tool spec without name', () => {
    const r = ToolSpecSchema.safeParse({
      description: 'noop',
      input_schema: { type: 'object', properties: {}, required: [] },
    })
    expect(r.success).toBe(false)
  })

  it('accepts a valid tool call', () => {
    const r = ToolCallSchema.safeParse({ name: 'search', input: { q: 'test' } })
    expect(r.success).toBe(true)
  })
})
