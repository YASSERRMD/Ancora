import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'

const TOOLS = [
  defineTool({ name: 'add', description: 'Add', schema: z.object({ a: z.number(), b: z.number() }), handler: ({ a, b }) => a + b }),
  defineTool({ name: 'concat', description: 'Concat', schema: z.object({ x: z.string(), y: z.string() }), handler: ({ x, y }) => x + y }),
  defineTool({ name: 'negate', description: 'Negate', schema: z.object({ n: z.number() }), handler: ({ n }) => -n }),
]

describe('ToolRegistry with multiple tools', () => {
  it('registers all tools', () => {
    const reg = new ToolRegistry()
    TOOLS.forEach((t) => reg.register(t))
    expect(reg.names).toHaveLength(3)
  })

  it('dispatches add correctly', async () => {
    const reg = new ToolRegistry()
    TOOLS.forEach((t) => reg.register(t))
    expect(await reg.dispatch('add', { a: 2, b: 3 })).toBe(5)
  })

  it('dispatches concat correctly', async () => {
    const reg = new ToolRegistry()
    TOOLS.forEach((t) => reg.register(t))
    expect(await reg.dispatch('concat', { x: 'foo', y: 'bar' })).toBe('foobar')
  })

  it('dispatches negate correctly', async () => {
    const reg = new ToolRegistry()
    TOOLS.forEach((t) => reg.register(t))
    expect(await reg.dispatch('negate', { n: 7 })).toBe(-7)
  })

  it('re-registering tool overwrites previous', async () => {
    const reg = new ToolRegistry()
    reg.register(defineTool({ name: 'f', description: 'v1', schema: z.object({}), handler: () => 1 }))
    reg.register(defineTool({ name: 'f', description: 'v2', schema: z.object({}), handler: () => 2 }))
    expect(reg.names).toHaveLength(1)
    expect(await reg.dispatch('f', {})).toBe(2)
  })
})
