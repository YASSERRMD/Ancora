import { z } from 'zod'
import { defineTool, ToolRegistry } from '../../tools'
import { ToolBridge, RunHandleLike, ToolBridgeEvent } from '../../tool-bridge'
import { RunEvent } from '../../schemas'

const addTool = defineTool({ name: 'add', description: 'add', schema: z.object({ a: z.number(), b: z.number() }), handler: ({ a, b }) => a + b })
const subTool = defineTool({ name: 'sub', description: 'sub', schema: z.object({ a: z.number(), b: z.number() }), handler: ({ a, b }) => a - b })
const mulTool = defineTool({ name: 'mul', description: 'mul', schema: z.object({ a: z.number(), b: z.number() }), handler: ({ a, b }) => a * b })

function makeHandle(events: RunEvent[]): RunHandleLike {
  const q = [...events]
  return {
    runId: 'mt',
    async *[Symbol.asyncIterator]() { for (const e of q) yield e },
    resume() {},
  }
}

describe('multiple tool calls conformance', () => {
  it('dispatches multiple sequential tool calls', async () => {
    const reg = new ToolRegistry()
    reg.register(addTool).register(subTool).register(mulTool)
    const bridge = new ToolBridge(reg)

    const handle = makeHandle([
      { kind: 'tool_call', run_id: 'r', name: 'add', input: '{"a":2,"b":3}' },
      { kind: 'tool_call', run_id: 'r', name: 'sub', input: '{"a":10,"b":4}' },
      { kind: 'tool_call', run_id: 'r', name: 'mul', input: '{"a":3,"b":7}' },
      { kind: 'completed', run_id: 'r' },
    ])

    const results: unknown[] = []
    for await (const ev of bridge.run(handle)) {
      if (ev.kind === 'tool_result') results.push(ev.result)
    }
    expect(results).toEqual([5, 6, 21])
  })

  it('ToolRegistry.specs returns all three tools', () => {
    const reg = new ToolRegistry()
    reg.register(addTool).register(subTool).register(mulTool)
    expect(reg.specs).toHaveLength(3)
    expect(reg.names.sort()).toEqual(['add', 'mul', 'sub'])
  })

  it('ToolRegistry.has returns false after only one registration', () => {
    const reg = new ToolRegistry()
    reg.register(addTool)
    expect(reg.has('add')).toBe(true)
    expect(reg.has('sub')).toBe(false)
    expect(reg.has('mul')).toBe(false)
  })

  it('bridge yields tool_result for each tool_call', async () => {
    const reg = new ToolRegistry()
    reg.register(addTool).register(mulTool)
    const bridge = new ToolBridge(reg)
    const events: ToolBridgeEvent[] = []
    for await (const ev of bridge.run(makeHandle([
      { kind: 'tool_call', run_id: 'r', name: 'add', input: '{"a":1,"b":1}' },
      { kind: 'tool_call', run_id: 'r', name: 'mul', input: '{"a":2,"b":2}' },
      { kind: 'completed', run_id: 'r' },
    ]))) {
      events.push(ev)
    }
    const toolResults = events.filter(e => e.kind === 'tool_result')
    expect(toolResults).toHaveLength(2)
  })
})
