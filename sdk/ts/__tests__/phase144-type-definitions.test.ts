jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(): string { return 'type-0' }
    pollRun(): Buffer | null { return null }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Runtime, version } from '../index'
import { Agent } from '../agent'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { AgentSpecSchema } from '../schemas'
import { zodToInputSchema } from '../zod-to-schema'
import { z } from 'zod'

describe('phase144 type definitions compile', () => {
  it('Runtime is a constructor function', () => {
    expect(typeof Runtime).toBe('function')
  })

  it('version is a function', () => {
    expect(typeof version).toBe('function')
  })

  it('Agent is a constructor function', () => {
    expect(typeof Agent).toBe('function')
  })

  it('defineTool is a function', () => {
    expect(typeof defineTool).toBe('function')
  })

  it('ToolRegistry is a constructor function', () => {
    expect(typeof ToolRegistry).toBe('function')
  })

  it('ToolBridge is a constructor function', () => {
    expect(typeof ToolBridge).toBe('function')
  })

  it('AgentSpecSchema is a zod schema', () => {
    expect(typeof AgentSpecSchema.safeParse).toBe('function')
  })

  it('zodToInputSchema is a function', () => {
    expect(typeof zodToInputSchema).toBe('function')
  })

  it('Runtime instance has isFreed getter', () => {
    const rt = new Runtime()
    expect(typeof rt.isFreed).toBe('boolean')
    rt.free()
  })

  it('Agent instance has run method', () => {
    const agent = new Agent()
    expect(typeof agent.run).toBe('function')
  })

  it('zodToInputSchema returns object with type', () => {
    const s = z.object({ name: z.string() })
    const result = zodToInputSchema(s)
    expect(result.type).toBe('object')
  })
})
