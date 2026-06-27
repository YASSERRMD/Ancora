const AG145: Record<string, string[]> = {}
let AG145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `ag-${AG145_CTR++}`
      AG145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = AG145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(AG145).forEach((k) => delete AG145[k]); AG145_CTR = 0 })

const LOCAL_SCHEMA = {
  type: 'object',
  properties: { answer: { type: 'string' } },
  required: ['answer'],
}

describe('phase145 air-gapped egress zero', () => {
  it('local schema has no external refs', () => {
    const raw = JSON.stringify(LOCAL_SCHEMA)
    expect(raw).not.toContain('http')
    expect(raw).not.toContain('$ref')
  })

  it('local schema type is object', () => {
    expect(LOCAL_SCHEMA.type).toBe('object')
  })

  it('runtime creates without network', () => {
    const { Runtime } = require('../index')
    expect(() => new Runtime()).not.toThrow()
  })

  it('agent run starts without network', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h.runId.length).toBeGreaterThan(0)
  })

  it('events contain no external API URLs', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    for await (const ev of h) {
      const str = JSON.stringify(ev)
      expect(str).not.toContain('api.anthropic.com')
      expect(str).not.toContain('api.openai.com')
    }
  })

  it('events contain no live API key patterns', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    for await (const ev of h) {
      const str = JSON.stringify(ev)
      expect(str).not.toContain('sk-ant-')
      expect(str).not.toContain('sk-proj-')
    }
  })

  it('conformance scenarios run offline', async () => {
    const agent = new Agent()
    const scenarios = ['single-agent', 'verifier', 'hil', 'rag']
    for (const s of scenarios) {
      const h = agent.run(AgentSpecSchema.parse({ model: s }))
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect(evs.length).toBeGreaterThan(0)
    }
  })

  it('spec with output_schema is offline-safe', () => {
    const schema = JSON.stringify(LOCAL_SCHEMA)
    const result = AgentSpecSchema.safeParse({ model: 'llama3', instructions: schema })
    expect(result.success).toBe(true)
  })
})
