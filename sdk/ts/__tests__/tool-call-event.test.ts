import { RunEventSchema } from '../schemas'

describe('tool_call event in RunEventSchema', () => {
  it('validates a tool_call event', () => {
    const result = RunEventSchema.safeParse({
      kind: 'tool_call',
      run_id: 'r1',
      name: 'search',
      input: '{"query":"test"}',
    })
    expect(result.success).toBe(true)
  })

  it('rejects tool_call missing name', () => {
    expect(RunEventSchema.safeParse({
      kind: 'tool_call',
      run_id: 'r1',
      input: '{}',
    }).success).toBe(false)
  })

  it('rejects tool_call missing input', () => {
    expect(RunEventSchema.safeParse({
      kind: 'tool_call',
      run_id: 'r1',
      name: 'search',
    }).success).toBe(false)
  })

  it('tool_call input is a JSON string', () => {
    const result = RunEventSchema.safeParse({
      kind: 'tool_call',
      run_id: 'r1',
      name: 'calc',
      input: '{"a":1,"b":2}',
    })
    expect(result.success).toBe(true)
    if (result.success && result.data.kind === 'tool_call') {
      expect(typeof result.data.input).toBe('string')
    }
  })
})
