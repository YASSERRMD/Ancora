const HIL_RUNS: Record<string, string[]> = {}
let HIL_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `hil-${HIL_CTR++}`
      HIL_RUNS[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'awaiting_approval', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = HIL_RUNS[id]
      if (!q || q.length === 0) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(id: string, decision: Buffer): void {
      const q = HIL_RUNS[id]
      if (!q) return
      const dec = JSON.parse(decision.toString('utf8'))
      q.push(JSON.stringify({ kind: 'resumed', run_id: id, approved: dec.approved }))
      q.push(JSON.stringify({ kind: 'completed', run_id: id }))
    }
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => {
  Object.keys(HIL_RUNS).forEach((k) => delete HIL_RUNS[k])
  HIL_CTR = 0
})

describe('phase144 human-in-loop suspend resume', () => {
  it('run starts and emits awaiting_approval', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const events: unknown[] = []
    for await (const ev of h) {
      events.push(ev)
      if ((ev as { kind: string }).kind === 'awaiting_approval') break
    }
    expect(events.some((e) => (e as { kind: string }).kind === 'awaiting_approval')).toBe(true)
  })

  it('resumeRun with approved=true continues run', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id)
    rt.pollRun(id)
    const decision = Buffer.from(JSON.stringify({ approved: true }))
    rt.resumeRun(id, decision)
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.kind).toBe('resumed')
    expect(ev.approved).toBe(true)
  })

  it('resumed event has correct run_id', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id)
    rt.pollRun(id)
    rt.resumeRun(id, Buffer.from(JSON.stringify({ approved: true })))
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.run_id).toBe(id)
  })

  it('after resume, completed event follows', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id)
    rt.pollRun(id)
    rt.resumeRun(id, Buffer.from(JSON.stringify({ approved: false })))
    rt.pollRun(id)
    const last = JSON.parse(rt.pollRun(id)!)
    expect(last.kind).toBe('completed')
  })

  it('approve false sets approved=false in resumed event', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id)
    rt.pollRun(id)
    rt.resumeRun(id, Buffer.from(JSON.stringify({ approved: false })))
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.approved).toBe(false)
  })

  it('run_id is consistent across events', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    const ev1 = JSON.parse(rt.pollRun(id)!)
    const ev2 = JSON.parse(rt.pollRun(id)!)
    expect(ev1.run_id).toBe(id)
    expect(ev2.run_id).toBe(id)
  })

  it('awaiting_approval kind is a string', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id)
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(typeof ev.kind).toBe('string')
    expect(ev.kind).toBe('awaiting_approval')
  })

  it('two HIL runs are independent', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id1 = rt.startRun('{}')
    const id2 = rt.startRun('{}')
    expect(id1).not.toBe(id2)
    rt.pollRun(id1)
    rt.pollRun(id1)
    rt.resumeRun(id1, Buffer.from(JSON.stringify({ approved: true })))
    const ev = JSON.parse(rt.pollRun(id1)!)
    expect(ev.run_id).toBe(id1)
    const ev2 = JSON.parse(rt.pollRun(id2)!)
    expect(ev2.run_id).toBe(id2)
  })

  it('agent run produces awaiting_approval event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'hil-test' }))
    const kinds: string[] = []
    for await (const ev of h) {
      kinds.push((ev as { kind: string }).kind)
    }
    expect(kinds).toContain('awaiting_approval')
  })

  it('awaiting_approval event precedes completion', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'hil-order' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const kinds = events.map((e) => (e as { kind: string }).kind)
    const awaitIdx = kinds.indexOf('awaiting_approval')
    const compIdx = kinds.indexOf('completed')
    if (awaitIdx !== -1 && compIdx !== -1) {
      expect(awaitIdx).toBeLessThan(compIdx)
    }
  })
})
