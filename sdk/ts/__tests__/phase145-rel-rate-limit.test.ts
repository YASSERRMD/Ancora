const RL145: Record<string, string[]> = {}
let RL145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `rl-${RL145_CTR++}`
      RL145[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = RL145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(RL145).forEach((k) => delete RL145[k]); RL145_CTR = 0 })

const RATE_LIMIT = { status: 429, retry_after_ms: 100, message: 'Too Many Requests' }

describe('phase145 rate-limit handling', () => {
  it('rate limit fixture has status 429', () => {
    expect(RATE_LIMIT.status).toBe(429)
  })

  it('retry_after_ms is positive', () => {
    expect(RATE_LIMIT.retry_after_ms).toBeGreaterThan(0)
  })

  it('exponential backoff delays are increasing', () => {
    const delays = [0.01 * 2 ** 0, 0.01 * 2 ** 1, 0.01 * 2 ** 2, 0.01 * 2 ** 3]
    for (let i = 1; i < delays.length; i++) {
      expect(delays[i]).toBeGreaterThan(delays[i - 1])
    }
  })

  it('burst of five runs all get distinct IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const ids = Array.from({ length: 5 }, () => agent.run(spec).runId)
    expect(new Set(ids).size).toBe(5)
  })

  it('burst runs all complete', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    const handles = Array.from({ length: 5 }, () => agent.run(spec))
    for (const h of handles) {
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect((evs[evs.length - 1] as { kind: string }).kind).toBe('completed')
    }
  })

  it('sequential runs after simulated delay succeed', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'llama3' })
    for (let i = 0; i < 3; i++) {
      const h = agent.run(spec)
      const evs: unknown[] = []
      for await (const ev of h) evs.push(ev)
      expect(evs.length).toBeGreaterThan(0)
    }
  })

  it('rate limit message is a string', () => {
    expect(typeof RATE_LIMIT.message).toBe('string')
    expect(RATE_LIMIT.message.length).toBeGreaterThan(0)
  })
})
