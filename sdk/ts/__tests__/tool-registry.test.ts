import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'

const SEARCH_TOOL = defineTool({
  name: 'search',
  description: 'Search',
  schema: z.object({ query: z.string() }),
  handler: ({ query }) => `results: ${query}`,
})

const CALC_TOOL = defineTool({
  name: 'calc',
  description: 'Add',
  schema: z.object({ a: z.number(), b: z.number() }),
  handler: ({ a, b }) => a + b,
})

describe('ToolRegistry', () => {
  it('constructs empty', () => {
    const reg = new ToolRegistry()
    expect(reg.names).toHaveLength(0)
  })

  it('register adds a tool', () => {
    const reg = new ToolRegistry()
    reg.register(SEARCH_TOOL)
    expect(reg.has('search')).toBe(true)
  })

  it('names returns registered tool names', () => {
    const reg = new ToolRegistry()
    reg.register(SEARCH_TOOL)
    reg.register(CALC_TOOL)
    expect(reg.names).toContain('search')
    expect(reg.names).toContain('calc')
  })

  it('specs returns ToolSpec for each registered tool', () => {
    const reg = new ToolRegistry()
    reg.register(SEARCH_TOOL)
    expect(reg.specs).toHaveLength(1)
    expect(reg.specs[0].name).toBe('search')
  })

  it('dispatch invokes the correct handler', async () => {
    const reg = new ToolRegistry()
    reg.register(SEARCH_TOOL)
    const result = await reg.dispatch('search', { query: 'hello' })
    expect(result).toBe('results: hello')
  })

  it('dispatch handles async handler', async () => {
    const asyncTool = defineTool({
      name: 'async',
      description: 'Async',
      schema: z.object({ x: z.string() }),
      handler: async ({ x }) => `async:${x}`,
    })
    const reg = new ToolRegistry()
    reg.register(asyncTool)
    const result = await reg.dispatch('async', { x: 'test' })
    expect(result).toBe('async:test')
  })

  it('dispatch throws for unknown tool', async () => {
    const reg = new ToolRegistry()
    await expect(reg.dispatch('nonexistent', {})).rejects.toThrow()
  })

  it('has returns false for unregistered tool', () => {
    const reg = new ToolRegistry()
    expect(reg.has('search')).toBe(false)
  })

  it('register is chainable', () => {
    const reg = new ToolRegistry()
    const returned = reg.register(SEARCH_TOOL).register(CALC_TOOL)
    expect(returned).toBe(reg)
    expect(reg.names).toHaveLength(2)
  })
})
