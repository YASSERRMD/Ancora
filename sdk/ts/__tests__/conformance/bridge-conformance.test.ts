import { z } from 'zod'
import { ToolBridge, createToolBridge, RunHandleLike } from '../../tool-bridge'
import { ToolRegistry, defineTool } from '../../tools'
import { RunEvent } from '../../schemas'

const multiplyTool = defineTool({
  name: 'multiply',
  description: 'multiply two numbers',
  schema: z.object({ x: z.number(), y: z.number() }),
  handler: ({ x, y }) => x * y,
})

function makeHandle(events: RunEvent[]): RunHandleLike {
  const queue = [...events]
  return {
    runId: 'bridge-run',
    async *[Symbol.asyncIterator]() {
      for (const ev of queue) yield ev
    },
    resume(_decision: string | Uint8Array) {},
  }
}

describe('ToolBridge conformance', () => {
  it('yields non-tool_call events unchanged', async () => {
    const reg = new ToolRegistry()
    const bridge = new ToolBridge(reg)
    const handle = makeHandle([
      { kind: 'started', run_id: 'r1', spec: '{}' },
      { kind: 'completed', run_id: 'r1' },
    ])
    const events = []
    for await (const ev of bridge.run(handle)) events.push(ev)
    expect(events[0].kind).toBe('started')
    expect(events[1].kind).toBe('completed')
  })

  it('intercepts tool_call and yields tool_result', async () => {
    const reg = new ToolRegistry().register(multiplyTool) as ToolRegistry
    const bridge = new ToolBridge(reg)
    const handle = makeHandle([
      { kind: 'tool_call', run_id: 'r2', name: 'multiply', input: '{"x":3,"y":4}' },
      { kind: 'completed', run_id: 'r2' },
    ])
    const events = []
    for await (const ev of bridge.run(handle)) events.push(ev.kind)
    expect(events).toContain('tool_result')
    expect(events).toContain('completed')
    expect(events).not.toContain('tool_call')
  })

  it('tool_result contains the dispatched result', async () => {
    const reg = new ToolRegistry().register(multiplyTool) as ToolRegistry
    const bridge = new ToolBridge(reg)
    const handle = makeHandle([
      { kind: 'tool_call', run_id: 'r3', name: 'multiply', input: '{"x":5,"y":6}' },
      { kind: 'completed', run_id: 'r3' },
    ])
    const results: unknown[] = []
    for await (const ev of bridge.run(handle)) {
      if (ev.kind === 'tool_result') results.push(ev.result)
    }
    expect(results[0]).toBe(30)
  })

  it('tool_result has correct name', async () => {
    const reg = new ToolRegistry().register(multiplyTool) as ToolRegistry
    const bridge = new ToolBridge(reg)
    const handle = makeHandle([
      { kind: 'tool_call', run_id: 'r4', name: 'multiply', input: '{"x":1,"y":1}' },
      { kind: 'completed', run_id: 'r4' },
    ])
    for await (const ev of bridge.run(handle)) {
      if (ev.kind === 'tool_result') expect(ev.name).toBe('multiply')
    }
  })

  it('returns error object for unknown tool', async () => {
    const reg = new ToolRegistry()
    const bridge = new ToolBridge(reg)
    const handle = makeHandle([
      { kind: 'tool_call', run_id: 'r5', name: 'nonexistent', input: '{}' },
      { kind: 'completed', run_id: 'r5' },
    ])
    for await (const ev of bridge.run(handle)) {
      if (ev.kind === 'tool_result') {
        expect((ev.result as Record<string, unknown>)['error']).toBeTruthy()
      }
    }
  })
})

describe('createToolBridge conformance', () => {
  it('creates a ToolBridge with registered tools', async () => {
    const bridge = createToolBridge(multiplyTool)
    expect(bridge.registry.has('multiply')).toBe(true)
  })

  it('created bridge dispatches tools correctly', async () => {
    const bridge = createToolBridge(multiplyTool)
    const handle = makeHandle([
      { kind: 'tool_call', run_id: 'r6', name: 'multiply', input: '{"x":7,"y":8}' },
      { kind: 'completed', run_id: 'r6' },
    ])
    for await (const ev of bridge.run(handle)) {
      if (ev.kind === 'tool_result') expect(ev.result).toBe(56)
    }
  })
})
