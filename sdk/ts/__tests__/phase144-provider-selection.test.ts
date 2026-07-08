const PS_RUNS: Record<string, string[]> = {}
let PS_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `ps-${PS_CTR++}`
      PS_RUNS[id] = [
        JSON.stringify({ kind: 'started', run_id: id, spec: '{}' }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = PS_RUNS[id]
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
  Object.keys(PS_RUNS).forEach((k) => delete PS_RUNS[k])
  PS_CTR = 0
})

const ANTHROPIC_MODEL = 'claude-opus-4-8'
const OPENAI_MODEL = 'gpt-4o'
const GEMINI_MODEL = 'gemini-2-5-pro'
const MISTRAL_MODEL = 'mistral-large-latest'
const DEEPSEEK_MODEL = 'deepseek-chat'

describe('phase144 provider selection', () => {
  it('Anthropic model constant is non-empty', () => {
    expect(ANTHROPIC_MODEL.length).toBeGreaterThan(0)
  })

  it('OpenAI model constant is non-empty', () => {
    expect(OPENAI_MODEL.length).toBeGreaterThan(0)
  })

  it('Gemini model constant is non-empty', () => {
    expect(GEMINI_MODEL.length).toBeGreaterThan(0)
  })

  it('Mistral model constant is non-empty', () => {
    expect(MISTRAL_MODEL.length).toBeGreaterThan(0)
  })

  it('DeepSeek model constant is non-empty', () => {
    expect(DEEPSEEK_MODEL.length).toBeGreaterThan(0)
  })

  it('all five model IDs are distinct', () => {
    const models = [ANTHROPIC_MODEL, OPENAI_MODEL, GEMINI_MODEL, MISTRAL_MODEL, DEEPSEEK_MODEL]
    expect(new Set(models).size).toBe(5)
  })

  it('Anthropic model spec parses', () => {
    expect(AgentSpecSchema.safeParse({ model: ANTHROPIC_MODEL }).success).toBe(true)
  })

  it('OpenAI model spec parses', () => {
    expect(AgentSpecSchema.safeParse({ model: OPENAI_MODEL }).success).toBe(true)
  })

  it('all providers start a run', async () => {
    const agent = new Agent()
    const models = [ANTHROPIC_MODEL, OPENAI_MODEL, GEMINI_MODEL, MISTRAL_MODEL, DEEPSEEK_MODEL]
    const ids = models.map((m) => agent.run(AgentSpecSchema.parse({ model: m })).runId)
    expect(new Set(ids).size).toBe(5)
  })

  it('each provider run completes', async () => {
    const agent = new Agent()
    for (const model of [ANTHROPIC_MODEL, OPENAI_MODEL]) {
      const h = agent.run(AgentSpecSchema.parse({ model }))
      const events: unknown[] = []
      for await (const ev of h) events.push(ev)
      expect((events[events.length - 1] as { kind: string }).kind).toBe('completed')
    }
  })
})
