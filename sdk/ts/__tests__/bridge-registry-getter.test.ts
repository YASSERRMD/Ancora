import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'

describe('ToolBridge.registry getter', () => {
  it('returns the registry passed to constructor', () => {
    const registry = new ToolRegistry()
    const bridge = new ToolBridge(registry)
    expect(bridge.registry).toBe(registry)
  })

  it('registry getter reflects registered tools', () => {
    const registry = new ToolRegistry()
    const tool = defineTool({ name: 't', description: 'd', schema: z.object({}), handler: () => null })
    registry.register(tool)
    const bridge = new ToolBridge(registry)
    expect(bridge.registry.has('t')).toBe(true)
  })
})
