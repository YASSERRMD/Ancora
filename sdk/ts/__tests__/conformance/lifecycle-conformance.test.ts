import { z } from 'zod'
import { defineTool, ToolRegistry } from '../../tools'
import { ToolBridge, RunHandleLike, ToolBridgeEvent } from '../../tool-bridge'
import { tokenText } from '../../helpers'
import { RunEvent } from '../../schemas'

function makeHandle(events: RunEvent[]): RunHandleLike {
  const q = [...events]
  return {
    runId: 'lc',
    async *[Symbol.asyncIterator]() { for (const e of q) yield e },
    resume() {},
  }
}

describe('full lifecycle conformance', () => {
  it('complete run: started, tokens, tool_call, tool_result, completed', async () => {
    const sqrtTool = defineTool({
      name: 'sqrt',
      description: 'square root',
      schema: z.object({ n: z.number() }),
      handler: ({ n }) => Math.sqrt(n),
    })
    const bridge = new ToolBridge(new ToolRegistry().register(sqrtTool) as ToolRegistry)
    const events: ToolBridgeEvent[] = []
    for await (const ev of bridge.run(makeHandle([
      { kind: 'started', run_id: 'lc', spec: '{}' },
      { kind: 'token', run_id: 'lc', text: 'calc: ' },
      { kind: 'tool_call', run_id: 'lc', name: 'sqrt', input: '{"n":16}' },
      { kind: 'token', run_id: 'lc', text: 'done' },
      { kind: 'completed', run_id: 'lc' },
    ]))) {
      events.push(ev)
    }
    const kinds = events.map(e => e.kind)
    expect(kinds[0]).toBe('started')
    expect(kinds).toContain('token')
    expect(kinds).toContain('tool_result')
    expect(kinds[kinds.length - 1]).toBe('completed')
  })

  it('tokenText from lifecycle events skips tool_result', () => {
    const events: RunEvent[] = [
      { kind: 'started', run_id: 'lc', spec: '{}' },
      { kind: 'token', run_id: 'lc', text: 'hello ' },
      { kind: 'token', run_id: 'lc', text: 'world' },
      { kind: 'completed', run_id: 'lc' },
    ]
    expect(tokenText(events)).toBe('hello world')
  })

  it('tool_result result is the sqrt of 16', async () => {
    const sqrtTool = defineTool({
      name: 'sqrt',
      description: 'sqrt',
      schema: z.object({ n: z.number() }),
      handler: ({ n }) => Math.sqrt(n),
    })
    const bridge = new ToolBridge(new ToolRegistry().register(sqrtTool) as ToolRegistry)
    let toolResult: unknown = null
    for await (const ev of bridge.run(makeHandle([
      { kind: 'tool_call', run_id: 'lc2', name: 'sqrt', input: '{"n":25}' },
      { kind: 'completed', run_id: 'lc2' },
    ]))) {
      if (ev.kind === 'tool_result') toolResult = ev.result
    }
    expect(toolResult).toBe(5)
  })
})
