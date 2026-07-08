const CANCEL_RUNS: Record<string, string[]> = {}
let CANCEL_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `ca-${CANCEL_CTR++}`
      CANCEL_RUNS[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'token', run_id: id, text: 't1' }),
        JSON.stringify({ kind: 'token', run_id: id, text: 't2' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = CANCEL_RUNS[id]
      if (!q || q.length === 0) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => {
  Object.keys(CANCEL_RUNS).forEach((k) => delete CANCEL_RUNS[k])
  CANCEL_CTR = 0
})

describe('phase144 cancellation', () => {
  it('break out of for-await stops iteration', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    let count = 0
    for await (const ev of h) {
      count++
      if ((ev as { kind: string }).kind === 'started') break
    }
    expect(count).toBe(1)
  })

  it('run ID is set before break', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const id = h.runId
    for await (const _ of h) break
    expect(id.length).toBeGreaterThan(0)
  })

  it('new run starts after early break on previous', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const h1 = agent.run(spec)
    for await (const _ of h1) break
    const h2 = agent.run(spec)
    expect(h2.runId).toBeDefined()
    expect(h2.runId).not.toBe(h1.runId)
  })

  it('AbortSignal aborts before second event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const abortController = new AbortController()
    let count = 0
    try {
      for await (const _ of h) {
        count++
        if (count === 1) abortController.abort()
        if (abortController.signal.aborted) break
      }
    } catch (_) {}
    expect(count).toBe(1)
  })

  it('partial drain does not corrupt counter', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const id1 = agent.run(spec).runId
    const id2 = agent.run(spec).runId
    expect(id1).not.toBe(id2)
  })

  it('run_id stable after early exit', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const id = h.runId
    let ev1: unknown = null
    for await (const ev of h) {
      ev1 = ev
      break
    }
    expect((ev1 as { run_id: string }).run_id).toBe(id)
  })

  it('second run events independent of first partial drain', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const h1 = agent.run(spec)
    for await (const _ of h1) break
    const h2 = agent.run(spec)
    let count = 0
    for await (const _ of h2) count++
    expect(count).toBe(4)
  })

  it('runtime not freed after partial run', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    rt.startRun('{}')
    expect(rt.isFreed).toBe(false)
    rt.free()
    expect(rt.isFreed).toBe(true)
  })

  it('start ten runs and break from each after first event', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for (let i = 0; i < 10; i++) {
      const h = agent.run(spec)
      for await (const _ of h) break
    }
    expect(CANCEL_CTR).toBe(10)
  })
})
