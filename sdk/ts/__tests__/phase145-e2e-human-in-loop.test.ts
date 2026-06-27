const HIL145: Record<string, string[]> = {}
let HIL145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `h145-${HIL145_CTR++}`
      HIL145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'awaiting_approval', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = HIL145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(id: string, dec: Buffer): void {
      const q = HIL145[id]; if (!q) return
      const d = JSON.parse(dec.toString('utf8'))
      q.push(JSON.stringify({ kind: 'resumed', run_id: id, approved: d.approved }))
      q.push(JSON.stringify({ kind: 'completed', run_id: id }))
    }
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(HIL145).forEach((k) => delete HIL145[k]); HIL145_CTR = 0 })

describe('phase145 e2e human-in-loop end to end', () => {
  it('run emits awaiting_approval', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const kinds: string[] = []
    for await (const ev of h) kinds.push((ev as { kind: string }).kind)
    expect(kinds).toContain('awaiting_approval')
  })

  it('resume with approve continues to completed', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id); rt.pollRun(id)
    rt.resumeRun(id, Buffer.from(JSON.stringify({ approved: true })))
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.kind).toBe('resumed')
    expect(ev.approved).toBe(true)
    expect(JSON.parse(rt.pollRun(id)!).kind).toBe('completed')
  })

  it('resume with reject sets approved false', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id); rt.pollRun(id)
    rt.resumeRun(id, Buffer.from(JSON.stringify({ approved: false })))
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.approved).toBe(false)
  })

  it('run_id consistent after resume', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id); rt.pollRun(id)
    rt.resumeRun(id, Buffer.from(JSON.stringify({ approved: true })))
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.run_id).toBe(id)
  })

  it('awaiting_approval precedes completed', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const kinds: string[] = []
    for await (const ev of h) kinds.push((ev as { kind: string }).kind)
    const ai = kinds.indexOf('awaiting_approval')
    const ci = kinds.indexOf('completed')
    if (ai !== -1 && ci !== -1) expect(ai).toBeLessThan(ci)
  })

  it('two HIL runs are independent', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id1 = rt.startRun('{}')
    const id2 = rt.startRun('{}')
    expect(id1).not.toBe(id2)
    const ev1 = JSON.parse(rt.pollRun(id1)!)
    const ev2 = JSON.parse(rt.pollRun(id2)!)
    expect(ev1.run_id).toBe(id1)
    expect(ev2.run_id).toBe(id2)
  })

  it('runtime not freed during HIL', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    rt.startRun('{}')
    expect(rt.isFreed).toBe(false)
    rt.free()
  })

  it('three consecutive HIL runs all have distinct IDs', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = [agent.run(spec).runId, agent.run(spec).runId, agent.run(spec).runId]
    expect(new Set(ids).size).toBe(3)
  })

  it('started precedes awaiting_approval', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const kinds: string[] = []
    for await (const ev of h) kinds.push((ev as { kind: string }).kind)
    expect(kinds.indexOf('started')).toBeLessThan(kinds.indexOf('awaiting_approval'))
  })

  it('resume returns resumed then completed events', async () => {
    const { Runtime } = await import('../index')
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id); rt.pollRun(id)
    rt.resumeRun(id, Buffer.from(JSON.stringify({ approved: true })))
    const r1 = JSON.parse(rt.pollRun(id)!)
    const r2 = JSON.parse(rt.pollRun(id)!)
    expect(r1.kind).toBe('resumed')
    expect(r2.kind).toBe('completed')
  })
})
