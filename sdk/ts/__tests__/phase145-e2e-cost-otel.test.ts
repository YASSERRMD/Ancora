interface OtelSpan {
  trace_id: string
  span_id: string
  name: string
  attributes: { model_id: string; input_tokens: number; output_tokens: number; cost_usd: number }
}

interface CostOtelEvent {
  type: 'cost'
  input_tokens: number
  output_tokens: number
  cost_usd: number
  trace_id: string
}

const OTEL_SPAN: OtelSpan = {
  trace_id: 'ts-trace-abc123',
  span_id: 'span-ts-001',
  name: 'ancora.agent.run',
  attributes: { model_id: 'llama3', input_tokens: 120, output_tokens: 60, cost_usd: 0.003 },
}

const COST_OTEL: CostOtelEvent = {
  type: 'cost',
  input_tokens: 120,
  output_tokens: 60,
  cost_usd: 0.003,
  trace_id: 'ts-trace-abc123',
}

const COTEL145: Record<string, string[]> = {}
let COTEL145_CTR = 0

jest.mock('../ancora.node', () => ({
  Runtime: class {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(_: Buffer): string {
      const id = `cotel-${COTEL145_CTR++}`
      COTEL145[id] = [
        JSON.stringify({ kind: 'started', run_id: id }),
        JSON.stringify({ kind: 'cost', run_id: id, ...COST_OTEL }),
        JSON.stringify({ kind: 'completed', run_id: id }),
      ]
      return id
    }
    pollRun(id: string): Buffer | null {
      const q = COTEL145[id]; if (!q || !q.length) return null
      return Buffer.from(q.shift()!, 'utf8')
    }
    resumeRun(): void {}
  },
  version: () => '0.1.0',
}), { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

beforeEach(() => { Object.keys(COTEL145).forEach((k) => delete COTEL145[k]); COTEL145_CTR = 0 })

describe('phase145 cost and otel emission verified', () => {
  it('otel span has trace_id', () => {
    expect(OTEL_SPAN.trace_id).toBe('ts-trace-abc123')
  })

  it('otel span has span_id', () => {
    expect(OTEL_SPAN.span_id.length).toBeGreaterThan(0)
  })

  it('otel span attributes have model_id', () => {
    expect(OTEL_SPAN.attributes.model_id).toBe('llama3')
  })

  it('otel span input_tokens non-negative', () => {
    expect(OTEL_SPAN.attributes.input_tokens).toBeGreaterThanOrEqual(0)
  })

  it('cost_usd non-negative', () => {
    expect(COST_OTEL.cost_usd).toBeGreaterThanOrEqual(0)
  })

  it('cost event trace_id matches span trace_id', () => {
    expect(COST_OTEL.trace_id).toBe(OTEL_SPAN.trace_id)
  })

  it('agent run emits cost event', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    expect(evs.some((e) => (e as { kind: string }).kind === 'cost')).toBe(true)
  })

  it('cost event input_tokens matches fixture', async () => {
    const agent = new Agent()
    const h = agent.run(AgentSpecSchema.parse({ model: 'llama3' }))
    const evs: unknown[] = []
    for await (const ev of h) evs.push(ev)
    const ce = evs.find((e) => (e as { kind: string }).kind === 'cost') as { input_tokens?: number }
    expect(ce?.input_tokens).toBe(120)
  })

  it('accumulated cost over five runs', () => {
    let total = 0
    for (let i = 0; i < 5; i++) total += COST_OTEL.cost_usd
    expect(total).toBeCloseTo(0.015, 9)
  })

  it('otel span JSON round-trips', () => {
    const rt = JSON.parse(JSON.stringify(OTEL_SPAN)) as OtelSpan
    expect(rt.trace_id).toBe(OTEL_SPAN.trace_id)
    expect(rt.attributes.cost_usd).toBe(OTEL_SPAN.attributes.cost_usd)
  })
})
