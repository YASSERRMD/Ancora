let WASM_CTR = 0
const WASM_RUNS: Record<string, string[]> = {}

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `wasm-${WASM_CTR++}`
      WASM_RUNS[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'wasm-ok' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = WASM_RUNS[id]
      if (!q || q.length === 0) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0-wasm',
}), { virtual: true })

import { Runtime, version } from '../index'
import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => {
  Object.keys(WASM_RUNS).forEach((k) => delete WASM_RUNS[k])
  WASM_CTR = 0
})

describe('phase144 wasm path basic run', () => {
  it('version returns wasm version', () => {
    expect(version()).toMatch(/0\.1\.0/)
  })

  it('Runtime creates without throwing', () => {
    expect(() => new Runtime()).not.toThrow()
  })

  it('startRun returns wasm-prefixed id', () => {
    const rt = new Runtime()
    const id = rt.startRun('{}')
    expect(id).toMatch(/^wasm-/)
  })

  it('wasm run first event is started', () => {
    const rt = new Runtime()
    const id = rt.startRun('{}')
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.kind).toBe('started')
  })

  it('wasm run has token event with wasm-ok text', () => {
    const rt = new Runtime()
    const id = rt.startRun('{}')
    rt.pollRun(id)
    const ev = JSON.parse(rt.pollRun(id)!)
    expect(ev.kind).toBe('token')
    expect(ev.text).toBe('wasm-ok')
  })

  it('wasm run completes', () => {
    const rt = new Runtime()
    const id = rt.startRun('{}')
    let last: string | null = null
    let ev = rt.pollRun(id)
    while (ev !== null) {
      last = ev
      ev = rt.pollRun(id)
    }
    expect(JSON.parse(last!).kind).toBe('completed')
  })

  it('Agent run over wasm path starts', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3-wasm' }))
    expect(h.runId).toMatch(/^wasm-/)
  })

  it('Agent run over wasm path completes', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3-wasm' }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('two wasm runs have distinct IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'wasm-model' })
    const id1 = agent.run(spec).runId
    const id2 = agent.run(spec).runId
    expect(id1).not.toBe(id2)
  })

  it('wasm runtime free sets isFreed', () => {
    const rt = new Runtime()
    rt.free()
    expect(rt.isFreed).toBe(true)
  })
})
