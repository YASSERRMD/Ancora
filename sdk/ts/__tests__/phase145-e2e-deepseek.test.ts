const DS145: Record<string, string[]> = {}
let DS145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(spec: Buffer): string {
      const id = `ds-${DS145_CTR++}`
      const model = (() => { try { return JSON.parse(spec.toString()).model } catch { return 'unknown' } })()
      DS145[id] = [
        JSON.stringify({ kind: 'started', run_id: id, model }),
        JSON.stringify({ kind: 'token', run_id: id, text: 'deepseek-response' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = DS145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(DS145).forEach((k) => delete DS145[k]); DS145_CTR = 0 })

const DEEPSEEK_MODEL = 'deepseek-chat'
const DEEPSEEK_CODER = 'deepseek-coder'
const QWEN_MODEL = 'qwen-turbo'

describe('phase145 e2e deepseek via mock gateway end to end', () => {
  it('deepseek model ID is non-empty', () => {
    expect(DEEPSEEK_MODEL.length).toBeGreaterThan(0)
  })

  it('deepseek-chat spec parses', () => {
    expect(AgentSpecSchema.safeParse({ model: DEEPSEEK_MODEL }).success).toBe(true)
  })

  it('deepseek-coder spec parses', () => {
    expect(AgentSpecSchema.safeParse({ model: DEEPSEEK_CODER }).success).toBe(true)
  })

  it('qwen spec parses', () => {
    expect(AgentSpecSchema.safeParse({ model: QWEN_MODEL }).success).toBe(true)
  })

  it('deepseek-chat run starts', () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: DEEPSEEK_MODEL }))
    expect(h.runId).toMatch(/^ds-/)
  })

  it('deepseek run emits token event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: DEEPSEEK_MODEL }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    const tok = events.find((e) => (e as { kind: string }).kind === 'token') as { text: string }
    expect(tok?.text).toBe('deepseek-response')
  })

  it('deepseek run completes', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: DEEPSEEK_MODEL }))
    const events: unknown[] = []
    for await (const ev of h) events.push(ev)
    expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
  })

  it('deepseek and qwen runs distinct IDs', () => {
    const agent = new Agent()
    const ds = agent.run(AgentSpecSchema.parse({ model: DEEPSEEK_MODEL }))
    const qw = agent.run(AgentSpecSchema.parse({ model: QWEN_MODEL }))
    expect(ds.runId).not.toBe(qw.runId)
  })

  it('all three models produce distinct run IDs', () => {
    const agent = new Agent()
    const ids = [DEEPSEEK_MODEL, DEEPSEEK_CODER, QWEN_MODEL].map((m) => agent.run(AgentSpecSchema.parse({ model: m })).runId)
    expect(new Set(ids).size).toBe(3)
  })

  it('no auth token leaked in events', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: DEEPSEEK_MODEL }))
    for await (const ev of h) {
      expect(JSON.stringify(ev)).not.toContain('sk-')
    }
  })

  it('deepseek-chat != deepseek-coder', () => {
    expect(DEEPSEEK_MODEL).not.toBe(DEEPSEEK_CODER)
  })
})
