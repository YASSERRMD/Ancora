const RAG145: Record<string, string[]> = {}
let RAG145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `rag-${RAG145_CTR++}`
      RAG145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'tool_call', run_id: id, name: 'qdrant_retrieve', input: '{"query":"vector","top_k":2}' }),
        JSON.stringify({ kind: 'tool_result', run_id: id, name: 'qdrant_retrieve', output: '[{"id":"q1","text":"Qdrant","score":0.95}]' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = RAG145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(RAG145).forEach((k) => delete RAG145[k]); RAG145_CTR = 0 })

const QDRANT_CHUNKS = [
  { id: 'q1', text: 'Qdrant supports cosine similarity.', score: 0.96 },
  { id: 'q2', text: 'Qdrant HNSW index is highly efficient.', score: 0.89 },
]

const qdrantTool = defineTool({
  name: 'qdrant_retrieve',
  description: 'RAG retrieve from Qdrant',
  schema: z.object({ query: z.string(), top_k: z.number().int().positive().default(2) }),
  handler: ({ top_k }) => JSON.stringify(QDRANT_CHUNKS.slice(0, top_k)),
})

describe('phase145 e2e rag with qdrant end to end', () => {
  it('qdrant tool has correct name', () => {
    expect(qdrantTool.name).toBe('qdrant_retrieve')
  })

  it('dispatch returns correct chunk count', () => {
    const reg = new ToolRegistry()
    reg.register(qdrantTool)
    const result = JSON.parse(reg.dispatch('qdrant_retrieve', { query: 'vector', top_k: 2 }) as string)
    expect(result).toHaveLength(2)
  })

  it('dispatch top_k=1 returns one chunk', () => {
    const reg = new ToolRegistry()
    reg.register(qdrantTool)
    const result = JSON.parse(reg.dispatch('qdrant_retrieve', { query: 'vector', top_k: 1 }) as string)
    expect(result).toHaveLength(1)
  })

  it('chunks have id, text, score', () => {
    QDRANT_CHUNKS.forEach((c) => {
      expect(c.id).toBeDefined()
      expect(c.text.length).toBeGreaterThan(0)
      expect(c.score).toBeGreaterThan(0)
    })
  })

  it('agent run emits tool_call event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [qdrantTool.spec] }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect(events.some((e) => (e as { kind: string }).kind === 'tool_call')).toBe(true)
  })

  it('agent run emits tool_result event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [qdrantTool.spec] }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect(events.some((e) => (e as { kind: string }).kind === 'tool_result')).toBe(true)
  })

  it('tool_call event has qdrant_retrieve name', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [qdrantTool.spec] }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const tc = events.find((e) => (e as { kind: string }).kind === 'tool_call') as { name: string }
    expect(tc?.name).toBe('qdrant_retrieve')
  })

  it('ToolBridge processes qdrant tool call', async () => {
    const reg = new ToolRegistry()
    reg.register(qdrantTool)
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [qdrantTool.spec] }))
    const bridge = new ToolBridge(reg)
    const events: unknown[] = []
    for await (const ev of bridge.run(h)) events.push(ev)
    expect(events.length).toBeGreaterThan(0)
  })

  it('run completes after tool result', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3', tools: [qdrantTool.spec] }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('fixture scores are descending', () => {
    for (let i = 1; i < QDRANT_CHUNKS.length; i++) {
      expect(QDRANT_CHUNKS[i].score).toBeLessThanOrEqual(QDRANT_CHUNKS[i - 1].score)
    }
  })

  it('two rag runs distinct IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3', tools: [qdrantTool.spec] })
    expect(agent.run(spec).runId).not.toBe(agent.run(spec).runId)
  })
})
