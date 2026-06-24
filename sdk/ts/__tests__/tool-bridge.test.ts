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
          JSON.stringify({ kind: 'tool_call', run_id: id, name: 'search', input: '{"query":"Ancora"}' }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'done' }),
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
        const q = runs[runId]
        if (!q) return
        q.push(JSON.stringify({ kind: 'resumed', run_id: runId, decision: decision.toString('utf8') }))
      }
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { z } from 'zod'
import { Agent } from '../agent'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { AgentSpecSchema } from '../schemas'

const searchTool = defineTool({
  name: 'search',
  description: 'Search',
  schema: z.object({ query: z.string() }),
  handler: ({ query }) => `results for ${query}`,
})

describe('ToolBridge', () => {
  it('ts tool runs in a run', async () => {
    const registry = new ToolRegistry()
    registry.register(searchTool)

    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({
      model: 'test',
      tools: [searchTool.spec],
    }))
    const bridge = new ToolBridge(registry)

    const events = []
    for await (const ev of bridge.run(handle)) {
      events.push(ev)
    }

    const toolResult = events.find((e) => e.kind === 'tool_result')
    expect(toolResult).toBeDefined()
    if (toolResult?.kind === 'tool_result') {
      expect(toolResult.name).toBe('search')
      expect(toolResult.result).toBe('results for Ancora')
    }
  })

  it('forwards non-tool-call events unchanged', async () => {
    const registry = new ToolRegistry()
    registry.register(searchTool)

    const agent = new Agent()
    const handle = agent.run(AgentSpecSchema.parse({ model: 'test' }))
    const bridge = new ToolBridge(registry)

    const events = []
    for await (const ev of bridge.run(handle)) {
      events.push(ev)
    }

    const started = events.find((e) => e.kind === 'started')
    const token = events.find((e) => e.kind === 'token')
    expect(started).toBeDefined()
    expect(token).toBeDefined()
  })

  it('run includes a completed event', async () => {
    const registry = new ToolRegistry()
    registry.register(searchTool)

    const agent = new Agent()
    const bridge = new ToolBridge(registry)
    const events = []
    for await (const ev of bridge.run(agent.run(AgentSpecSchema.parse({ model: 'test' })))) {
      events.push(ev)
    }
    expect(events.some((e) => e.kind === 'completed')).toBe(true)
  })
})
