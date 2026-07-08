const E2E_VR: Record<string, string[]> = {}
let E2E_VR_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `vr-${E2E_VR_CTR++}`
      E2E_VR[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'verified' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = E2E_VR[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(E2E_VR).forEach((k) => delete E2E_VR[k]); E2E_VR_CTR = 0 })

describe('phase145 e2e verifier end to end', () => {
  it('drafter and verifier run IDs are distinct', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const d = agent.run(spec)
    const v = agent.run(spec)
    expect(d.runId).not.toBe(v.runId)
  })

  it('drafter emits completed', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const evs: unknown[] = []
    for await (const e of h) evs.push(e)
    expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('verifier emits completed', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for await (const _ of agent.run(spec)) {}
    const v = agent.run(spec)
    const evs: unknown[] = []
    for await (const e of v) evs.push(e)
    expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('drafter events contain run_id matching handle', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const id = h.runId
    for await (const ev of h) {
      expect((ev as { run_id: string }).run_id).toBe(id)
    }
  })

  it('three-node pipeline produces three distinct run IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = [agent.run(spec).runId, agent.run(spec).runId, agent.run(spec).runId]
    expect(new Set(ids).size).toBe(3)
  })

  it('verifier and drafter events are independent', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const d = agent.run(spec)
    const v = agent.run(spec)
    const de: unknown[] = [], ve: unknown[] = []
    for await (const e of d) de.push(e)
    for await (const e of v) ve.push(e)
    expect((de[0] as { run_id: string }).run_id).toBe(d.runId)
    expect((ve[0] as { run_id: string }).run_id).toBe(v.runId)
  })

  it('verifier token text is verified', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    for await (const _ of h) {}
    const v = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const evs: unknown[] = []
    for await (const e of v) evs.push(e)
    const tok = evs.find((e) => (e as { kind: string }).kind === 'token') as { text: string }
    expect(tok?.text).toBe('verified')
  })

  it('multiple verifier cycles all complete', async () => {
    const agent = new Agent()
    for (let i = 0; i < 3; i++) {
      const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
      const evs: unknown[] = []
      for await (const e of h) evs.push(e)
      expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
    }
  })

  it('drafter run ID starts with vr prefix', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    expect(h.runId).toMatch(/^vr-/)
  })

  it('parallel drafter and verifier both complete', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const d = agent.run(spec)
    const v = agent.run(spec)
    const collect = async (h: typeof d) => { const a: unknown[] = []; for await (const e of h) a.push(e); return a }
    const [de, ve] = await Promise.all([collect(d), collect(v)])
    expect((de[de.length - 1] as { kind: string }).kind).toBe('completed')
    expect((ve[ve.length - 1] as { kind: string }).kind).toBe('completed')
  })
})
