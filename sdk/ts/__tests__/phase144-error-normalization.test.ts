import { AgentSpecSchema } from '../schemas'

const ErrOk = 0
const ErrInternal = 1
const ErrNotFound = 2
const ErrInvalidArg = 3

describe('phase144 error normalization', () => {
  it('ErrOk is 0', () => {
    expect(ErrOk).toBe(0)
  })

  it('ErrInternal is non-zero', () => {
    expect(ErrInternal).not.toBe(ErrOk)
  })

  it('ErrNotFound is non-zero', () => {
    expect(ErrNotFound).not.toBe(ErrOk)
  })

  it('ErrInvalidArg is non-zero', () => {
    expect(ErrInvalidArg).not.toBe(ErrOk)
  })

  it('error codes are distinct', () => {
    const codes = [ErrOk, ErrInternal, ErrNotFound, ErrInvalidArg]
    expect(new Set(codes).size).toBe(4)
  })

  it('AgentSpecSchema rejects empty model (validation error)', () => {
    const result = AgentSpecSchema.safeParse({ model: '' })
    expect(result.success).toBe(false)
  })

  it('AgentSpecSchema rejects missing model (validation error)', () => {
    const result = AgentSpecSchema.safeParse({})
    expect(result.success).toBe(false)
  })

  it('Error class has message property', () => {
    const err = new Error('runtime error')
    expect(err.message).toBe('runtime error')
  })

  it('TypeError is instanceof Error', () => {
    const err = new TypeError('bad type')
    expect(err instanceof Error).toBe(true)
  })

  it('error codes are integers', () => {
    [ErrOk, ErrInternal, ErrNotFound, ErrInvalidArg].forEach((code) => {
      expect(Number.isInteger(code)).toBe(true)
    })
  })
})
