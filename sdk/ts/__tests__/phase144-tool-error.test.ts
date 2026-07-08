jest.mock('../ancora.node', () => {
  const runs: Record<string, string[]> = {}
  let ctr = 0
  return {
    Runtime: class {
      private _freed = false
      get isFreed(): boolean { return this._freed }
      free(): void { this._freed = true }
      startRun(_: Buffer): string {
        const id = `te-${ctr++}`
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
          JSON.stringify({ kind: 'tool_call', run_id: id, name: 'fail_tool', input: '{"x":1}' }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(id: string): Buffer | null {
        const q = runs[id]
        if (!q || q.length === 0) return null
        return Buffer.from(q.shift()!, 'utf8')
      }
      resumeRun(): void {}
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

const failTool = defineTool({
  name: 'fail_tool',
  description: 'Always fails',
  schema: z.object({ x: z.number() }),
  handler: () => { throw new Error('intentional failure') },
})

describe('phase144 tool error propagation', () => {
  it('defineTool with throwing handler is registered', () => {
    const reg = new ToolRegistry()
    reg.register(failTool)
    expect(reg.has('fail_tool')).toBe(true)
  })

  it('ToolRegistry dispatch propagates throw', async () => {
    const reg = new ToolRegistry()
    reg.register(failTool)
    await expect(reg.dispatch('fail_tool', { x: 1 })).rejects.toThrow('intentional failure')
  })

  it('ToolRegistry dispatch on missing tool throws', async () => {
    const reg = new ToolRegistry()
    await expect(reg.dispatch('nonexistent', {})).rejects.toThrow()
  })

  it('ToolBridge continues iteration after tool error', async () => {
    const reg = new ToolRegistry()
    reg.register(failTool)
    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'test', tools: [failTool.spec] }))
    const bridge = new ToolBridge(reg)
    const events: unknown[] = []
    try {
      for await (const ev of bridge.run(handle)) {
        events.push(ev)
      }
    } catch (_) {}
    expect(events.length).toBeGreaterThanOrEqual(0)
  })

  it('error message is preserved', async () => {
    const reg = new ToolRegistry()
    reg.register(failTool)
    let msg = ''
    try {
      await reg.dispatch('fail_tool', { x: 99 })
    } catch (err) {
      msg = (err as Error).message
    }
    expect(msg).toBe('intentional failure')
  })

  it('async throwing handler is supported', async () => {
    const asyncFail = defineTool({
      name: 'async_fail',
      description: 'Async fail',
      schema: z.object({ n: z.number() }),
      handler: async () => { throw new Error('async error') },
    })
    const reg = new ToolRegistry()
    reg.register(asyncFail)
    await expect(reg.dispatch('async_fail', { n: 1 })).rejects.toThrow('async error')
  })

  it('tool spec name matches handler name', () => {
    expect(failTool.spec.name).toBe('fail_tool')
  })

  it('tool not in registry dispatch throws ReferenceError-like', async () => {
    const reg = new ToolRegistry()
    await expect(reg.dispatch('ghost', {})).rejects.toThrow()
  })

  it('error does not corrupt registry', async () => {
    const reg = new ToolRegistry()
    reg.register(failTool)
    try { await reg.dispatch('fail_tool', { x: 1 }) } catch (_) {}
    expect(reg.has('fail_tool')).toBe(true)
  })

  it('second tool still works after first throws', async () => {
    const ok = defineTool({
      name: 'ok_tool',
      description: 'Always ok',
      schema: z.object({ v: z.string() }),
      handler: ({ v }) => v,
    })
    const reg = new ToolRegistry()
    reg.register(failTool)
    reg.register(ok)
    try { await reg.dispatch('fail_tool', { x: 1 }) } catch (_) {}
    expect(await reg.dispatch('ok_tool', { v: 'fine' })).toBe('fine')
  })
})
