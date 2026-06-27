const WASM145: Record<string, string[]> = {}
let WASM145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(spec: Buffer): string {
      const id = `wasm145-${WASM145_CTR++}`
      const model = (() => { try { return JSON.parse(spec.toString()).model } catch { return 'wasm' } })()
      WASM145[id] = [
        JSON.stringify({ kind: 'started', run_id: id, transport: 'wasm', model }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'wasm-via-sidecar' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = WASM145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0-wasm',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(WASM145).forEach((k) => delete WASM145[k]); WASM145_CTR = 0 })

describe('phase145 wasm client drives a run via sidecar', () => {
  it('wasm run ID starts with wasm145 prefix', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3-wasm' }))
    expect(h.runId).toMatch(/^wasm145-/)
  })

  it('started event has transport wasm', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3-wasm' }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    const started = evs[0] as { transport?: string }
    expect(started.transport).toBe('wasm')
  })

  it('wasm run emits token with wasm-via-sidecar', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3-wasm' }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    const tok = evs.find((e) => (e as { kind: string }).kind === 'token') as { text: string }
    expect(tok?.text).toBe('wasm-via-sidecar')
  })

  it('wasm run completes', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3-wasm' }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('two wasm runs have distinct IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'wasm-model' })
    expect(agent.run(spec).runId).not.toBe(agent.run(spec).runId)
  })

  it('wasm run produces exactly 3 events', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'wasm-3ev' }))
    let count = 0
    for await (const _ of h) count++
    expect(count).toBe(3)
  })

  it('started event model matches spec model', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'my-wasm-model' }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    const started = evs[0] as { model?: string }
    expect(started.model).toBe('my-wasm-model')
  })

  it('wasm counter increments monotonically', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'wasm' })
    const ids = [agent.run(spec).runId, agent.run(spec).runId]
    const n1 = parseInt(ids[0].replace('wasm145-', ''))
    const n2 = parseInt(ids[1].replace('wasm145-', ''))
    expect(n1).toBeLessThan(n2)
  })
})
