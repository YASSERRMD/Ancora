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
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec: specBytes.toString('utf8') }),
          JSON.stringify({ kind: 'tool_call', run_id: id, name: 'unknown_tool', input: '{}' }),
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

import { Agent } from '../agent'
import { ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { AgentSpecSchema } from '../schemas'

describe('ToolBridge error handling', () => {
  it('unknown tool yields tool_result with error field', async () => {
    const registry = new ToolRegistry()
    const agent = new Agent()
    const bridge = new ToolBridge(registry)
    const events = []
    for await (const ev of bridge.run(agent.run(AgentSpecSchema.parse({ model: 'test' })))) {
      events.push(ev)
    }
    const toolResult = events.find((e) => e.kind === 'tool_result')
    expect(toolResult).toBeDefined()
    if (toolResult?.kind === 'tool_result') {
      expect(toolResult.result).toMatchObject({ error: expect.any(String) })
    }
  })

  it('run completes even after unknown tool call', async () => {
    const registry = new ToolRegistry()
    const agent = new Agent()
    const bridge = new ToolBridge(registry)
    const events = []
    for await (const ev of bridge.run(agent.run(AgentSpecSchema.parse({ model: 'test' })))) {
      events.push(ev)
    }
    expect(events.some((e) => e.kind === 'completed')).toBe(true)
  })
})
