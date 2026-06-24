jest.mock('../ancora.node', () => {
  const runs: Record<string, string[]> = {}
  let counter = 0
  return {
    Runtime: class MockRuntime {
      private _freed = false
      get isFreed(): boolean { return this._freed }
      free(): void { this._freed = true }
      startRun(specBytes: Buffer): string {
        const id = `run-${counter++}`
        const spec = JSON.parse(specBytes.toString('utf8'))
        const toolNames = (spec.tools || []).map((t: { name: string }) => t.name).join(',')
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec: specBytes.toString('utf8') }),
          JSON.stringify({ kind: 'tool_call', run_id: id, name: 'calc', input: '{"a":10,"b":5}' }),
          JSON.stringify({ kind: 'token', run_id: id, text: `tools:${toolNames}` }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(runId: string): Buffer | null {
        const q = runs[runId]
        if (!q || q.length === 0) return null
        return Buffer.from(q.shift()!, 'utf8')
      }
      resumeRun(runId: string, decision: Buffer): void {
        runs[runId]?.push(JSON.stringify({ kind: 'resumed', run_id: runId, decision: decision.toString('utf8') }))
      }
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { z } from 'zod'
import { Agent } from '../agent'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { buildSpec } from '../wire'
import { collectEvents, tokenText } from '../helpers'

describe('full tools integration', () => {
  it('define -> registry -> bridge -> run shows tool result and token text', async () => {
    const calcTool = defineTool({
      name: 'calc',
      description: 'Add two numbers',
      schema: z.object({ a: z.number(), b: z.number() }),
      handler: ({ a, b }) => a + b,
    })

    const registry = new ToolRegistry().register(calcTool)
    const spec = buildSpec('test', { tools: registry.specs })
    const agent = new Agent()
    const bridge = new ToolBridge(registry)

    const allEvents = []
    for await (const ev of bridge.run(agent.run(spec))) {
      allEvents.push(ev)
    }

    const toolResult = allEvents.find((e) => e.kind === 'tool_result')
    expect(toolResult?.kind === 'tool_result' && toolResult.result).toBe(15)

    const tokenEvents = allEvents.filter((e) => e.kind === 'token')
    const text = tokenEvents
      .map((e) => (e.kind === 'token' ? e.text : ''))
      .join('')
    expect(text).toContain('calc')

    expect(allEvents.some((e) => e.kind === 'completed')).toBe(true)
    agent.free()
  })
})
