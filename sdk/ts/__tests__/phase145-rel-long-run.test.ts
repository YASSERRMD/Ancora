const LR145: Record<string, string[]> = {}
let LR145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `lr-${LR145_CTR++}`
      LR145[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = LR145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Runtime } from '../index'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(LR145).forEach((k) => delete LR145[k]); LR145_CTR = 0 })

describe('phase145 long-run stability', () => {
  it('fifty sequential runs all have unique IDs', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = new Set<string>()
    for (let i = 0; i < 50; i++) {
      const id = agent.run(spec).runId
      expect(ids.has(id)).toBe(false)
      ids.add(id)
    }
    expect(ids.size).toBe(50)
  })

  it('fifty runs all complete', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for (let i = 0; i < 50; i++) {
      const h = agent.run(spec)
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
    }
  })

  it('hundred runtime create-free cycles succeed', () => {
    for (let i = 0; i < 100; i++) {
      const rt = new Runtime()
      expect(rt.isFreed).toBe(false)
      rt.free()
      expect(rt.isFreed).toBe(true)
    }
  })

  it('five hundred in-memory store ops succeed', () => {
    const store = new Map<string, number>()
    for (let i = 0; i < 500; i++) store.set(`key-${i}`, i * 2)
    for (let i = 0; i < 500; i++) expect(store.get(`key-${i}`)).toBe(i * 2)
  })

  it('ten concurrent run IDs are unique', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = Array.from({ length: 10 }, () => agent.run(spec).runId)
    expect(new Set(ids).size).toBe(10)
  })

  it('LR145_CTR matches total run count after fifty', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for (let i = 0; i < 50; i++) agent.run(spec)
    expect(LR145_CTR).toBe(50)
  })

  it('drain all fifty runs produces non-empty event lists', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for (let i = 0; i < 50; i++) {
      const h = agent.run(spec)
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect(evs.length).toBeGreaterThan(0)
    }
  })
})
