import { z } from 'zod'
import { defineTool, ToolRegistry } from '../../tools'
import { ToolBridge, RunHandleLike } from '../../tool-bridge'
import { RunEvent } from '../../schemas'
import { parseEvent } from '../../wire'

function makeHandle(events: RunEvent[]): RunHandleLike {
  const q = [...events]
  return {
    runId: 'err',
    async *[Symbol.asyncIterator]() { for (const e of q) yield e },
    resume() {},
  }
}

describe('error handling conformance', () => {
  it('ToolBridge wraps handler exceptions as error result', async () => {
    const crashTool = defineTool({
      name: 'crash',
      description: 'always throws',
      schema: z.object({}),
      handler: () => { throw new Error('kaboom') },
    })
    const reg = new ToolRegistry().register(crashTool) as ToolRegistry
    const bridge = new ToolBridge(reg)
    const events = []
    for await (const ev of bridge.run(makeHandle([
      { kind: 'tool_call', run_id: 'r', name: 'crash', input: '{}' },
      { kind: 'completed', run_id: 'r' },
    ]))) {
      events.push(ev)
    }
    const tr = events.find(e => e.kind === 'tool_result') as { kind: 'tool_result'; result: unknown } | undefined
    expect(tr).toBeDefined()
    expect((tr?.result as Record<string, unknown>)['error']).toContain('kaboom')
  })

  it('ToolBridge continues after handler error', async () => {
    const crashTool = defineTool({
      name: 'crash',
      description: 'always throws',
      schema: z.object({}),
      handler: () => { throw new Error('oops') },
    })
    const reg = new ToolRegistry().register(crashTool) as ToolRegistry
    const bridge = new ToolBridge(reg)
    const kinds: string[] = []
    for await (const ev of bridge.run(makeHandle([
      { kind: 'tool_call', run_id: 'r', name: 'crash', input: '{}' },
      { kind: 'completed', run_id: 'r' },
    ]))) {
      kinds.push(ev.kind)
    }
    expect(kinds).toContain('tool_result')
    expect(kinds).toContain('completed')
  })

  it('parseEvent throws a meaningful error for unknown kind', () => {
    let error: Error | null = null
    try {
      parseEvent('{"kind":"unknown_event","run_id":"r"}')
    } catch (e) {
      error = e as Error
    }
    expect(error).not.toBeNull()
    expect(error?.message).toBeTruthy()
  })

  it('ToolRegistry.dispatch throws for unknown tool with descriptive error', async () => {
    const reg = new ToolRegistry()
    let error: Error | null = null
    try {
      await reg.dispatch('missing', {})
    } catch (e) {
      error = e as Error
    }
    expect(error).not.toBeNull()
    expect(error?.message).toContain('missing')
  })
})
