const E2E_SA: Record<string, string[]> = {}
let E2E_SA_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `e2e-sa-${E2E_SA_CTR++}`
      E2E_SA[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'hi' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = E2E_SA[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(E2E_SA).forEach((k) => delete E2E_SA[k]); E2E_SA_CTR = 0 })

describe('phase145 e2e single agent end to end', () => {
  it('run produces a non-empty run ID', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h.runId.length).toBeGreaterThan(0)
  })

  it('run emits started event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect(events[0]).toMatchObject({ kind: 'started' })
  })

  it('run emits token event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect(events.some((e) => (e as { kind: string }).kind === 'token')).toBe(true)
  })

  it('run ends with completed event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('two consecutive runs have distinct IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    expect(agent.run(spec).runId).not.toBe(agent.run(spec).runId)
  })

  it('run started event run_id matches handle runId', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect((events[0] as { run_id: string }).run_id).toBe(h.runId)
  })

  it('repeated drain yields no more events second time', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    while (rt.pollRun(id) !== null) {}
    expect(rt.pollRun(id)).toBeNull()
  })

  it('runtime not freed during run', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    rt.startRun('{}')
    expect(rt.isFreed).toBe(false)
    rt.free()
  })

  it('run produces exactly 3 events', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    let count = 0
    for await (const _ of h) count++
    expect(count).toBe(3)
  })

  it('token text is hi', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const tok = events.find((e) => (e as { kind: string }).kind === 'token') as { text: string }
    expect(tok?.text).toBe('hi')
  })
})
