import { RunEventSchema } from '../schemas'

describe('RunEventSchema', () => {
  it('validates started event', () => {
    const result = RunEventSchema.safeParse({
      kind: 'started',
      run_id: 'abc',
      spec: '{"model":"gpt-4"}',
    })
    expect(result.success).toBe(true)
  })

  it('validates token event', () => {
    const result = RunEventSchema.safeParse({
      kind: 'token',
      run_id: 'abc',
      text: 'Hello',
    })
    expect(result.success).toBe(true)
  })

  it('validates completed event', () => {
    const result = RunEventSchema.safeParse({ kind: 'completed', run_id: 'abc' })
    expect(result.success).toBe(true)
  })

  it('validates resumed event', () => {
    const result = RunEventSchema.safeParse({
      kind: 'resumed',
      run_id: 'abc',
      decision: 'approve',
    })
    expect(result.success).toBe(true)
  })

  it('rejects unknown kind', () => {
    expect(RunEventSchema.safeParse({ kind: 'error', run_id: 'abc' }).success).toBe(false)
  })

  it('rejects started event missing spec', () => {
    expect(RunEventSchema.safeParse({ kind: 'started', run_id: 'abc' }).success).toBe(false)
  })

  it('rejects token event missing text', () => {
    expect(RunEventSchema.safeParse({ kind: 'token', run_id: 'abc' }).success).toBe(false)
  })

  it('rejects resumed event missing decision', () => {
    expect(RunEventSchema.safeParse({ kind: 'resumed', run_id: 'abc' }).success).toBe(false)
  })

  it('rejects event missing run_id', () => {
    expect(RunEventSchema.safeParse({ kind: 'completed' }).success).toBe(false)
  })
})
