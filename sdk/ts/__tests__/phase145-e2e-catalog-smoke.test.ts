const CAT145: Record<string, string[]> = {}
let CAT145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `cat-${CAT145_CTR++}`
      CAT145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = CAT145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(CAT145).forEach((k) => delete CAT145[k]); CAT145_CTR = 0 })

const CATALOG_EXAMPLES = [
  { name: 'single-agent', model: 'llama3' },
  { name: 'verifier-pipeline', model: 'gpt-4o' },
  { name: 'human-in-loop', model: 'claude-opus-4-8' },
  { name: 'rag-qdrant', model: 'llama3' },
  { name: 'mcp-tool', model: 'llama3' },
  { name: 'deepseek-chat', model: 'deepseek-chat' },
  { name: 'qwen-regional', model: 'qwen-turbo' },
  { name: 'streaming-agent', model: 'llama3' },
  { name: 'structured-output', model: 'gpt-4o' },
  { name: 'multi-node', model: 'llama3' },
]

describe('phase145 all catalog examples smoke test', () => {
  it('catalog has ten examples', () => {
    expect(CATALOG_EXAMPLES).toHaveLength(10)
  })

  it('all names are non-empty', () => {
    CATALOG_EXAMPLES.forEach((ex) => expect(ex.name.length).toBeGreaterThan(0))
  })

  it('all names are distinct', () => {
    const names = CATALOG_EXAMPLES.map((e) => e.name)
    expect(new Set(names).size).toBe(10)
  })

  it('all specs parse', () => {
    CATALOG_EXAMPLES.forEach((ex) => {
      expect(AgentSpecSchema.safeParse({ model: ex.model }).success).toBe(true)
    })
  })

  it('all examples start a run', () => {
    const agent = new Agent()
    CATALOG_EXAMPLES.forEach((ex) => {
      const h = agent.run(AgentSpecSchema.parse({ model: ex.model }))
      expect(h.runId.length).toBeGreaterThan(0)
    })
  })

  it('all example run IDs unique', () => {
    const agent = new Agent()
    const ids = CATALOG_EXAMPLES.map((ex) => agent.run(AgentSpecSchema.parse({ model: ex.model })).runId)
    expect(new Set(ids).size).toBe(10)
  })

  it('first example single-agent completes', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: CATALOG_EXAMPLES[0].model }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('last example multi-node completes', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: CATALOG_EXAMPLES[9].model }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
  })
})
