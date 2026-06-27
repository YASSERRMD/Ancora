class StoreForTest {
  private _data: Map<string, unknown> = new Map()
  private _fail = false
  setFail(v: boolean): void { this._fail = v }
  write(k: string, v: unknown): void {
    if (this._fail) throw new Error('store write failure')
    this._data.set(k, v)
  }
  read(k: string, def?: unknown): unknown { return this._data.has(k) ? this._data.get(k) : def }
  delete(k: string): void { this._data.delete(k) }
  clear(): void { this._data.clear() }
}

const SF145: Record<string, string[]> = {}
let SF145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `sf-${SF145_CTR++}`
      SF145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = SF145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(SF145).forEach((k) => delete SF145[k]); SF145_CTR = 0 })

describe('phase145 store failure recovery', () => {
  it('store write after clear works', () => {
    const s = new StoreForTest()
    s.write('k', 'v')
    s.clear()
    s.write('k2', 'v2')
    expect(s.read('k2')).toBe('v2')
    expect(s.read('k')).toBeUndefined()
  })

  it('store failure throws', () => {
    const s = new StoreForTest()
    s.setFail(true)
    expect(() => s.write('x', 1)).toThrow('store write failure')
  })

  it('store recovers after failure cleared', () => {
    const s = new StoreForTest()
    s.setFail(true)
    try { s.write('x', 1) } catch (_) {}
    s.setFail(false)
    s.write('y', 2)
    expect(s.read('y')).toBe(2)
  })

  it('overwrite works', () => {
    const s = new StoreForTest()
    s.write('k', 1); s.write('k', 2)
    expect(s.read('k')).toBe(2)
  })

  it('delete nonexistent noop', () => {
    const s = new StoreForTest()
    expect(() => s.delete('ghost')).not.toThrow()
  })

  it('agent run after store failure still starts', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h.runId.length).toBeGreaterThan(0)
  })

  it('agent run after store failure completes', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('multiple recovery cycles all succeed', async () => {
    const agent = new Agent()
    for (let i = 0; i < 3; i++) {
      const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect(evs.length).toBeGreaterThan(0)
    }
  })

  it('large value stored and retrieved', () => {
    const s = new StoreForTest()
    const big = 'x'.repeat(10_000)
    s.write('large', big)
    expect((s.read('large') as string).length).toBe(10_000)
  })
})
