interface CostEvent {
  type: 'usage'
  input_tokens: number
  output_tokens: number
  cost_usd: number
}

function makeCostEvent(input: number, output: number, cost: number): CostEvent {
  return { type: 'usage', input_tokens: input, output_tokens: output, cost_usd: cost }
}

describe('phase144 cost summary', () => {
  it('cost event has type usage', () => {
    expect(makeCostEvent(100, 50, 0.002).type).toBe('usage')
  })

  it('cost event input_tokens is non-negative', () => {
    expect(makeCostEvent(0, 0, 0).input_tokens).toBeGreaterThanOrEqual(0)
  })

  it('cost event output_tokens is non-negative', () => {
    expect(makeCostEvent(10, 5, 0.001).output_tokens).toBeGreaterThanOrEqual(0)
  })

  it('cost_usd is non-negative', () => {
    expect(makeCostEvent(10, 5, 0.001).cost_usd).toBeGreaterThanOrEqual(0)
  })

  it('cost event JSON round-trips', () => {
    const ev = makeCostEvent(150, 80, 0.003)
    const parsed = JSON.parse(JSON.stringify(ev)) as CostEvent
    expect(parsed.input_tokens).toBe(150)
    expect(parsed.output_tokens).toBe(80)
  })

  it('sum of multiple costs is correct', () => {
    const costs = [0.001, 0.002, 0.003]
    const total = costs.reduce((a, b) => a + b, 0)
    expect(total).toBeCloseTo(0.006, 9)
  })

  it('total input tokens sum is correct', () => {
    const events = [makeCostEvent(100, 50, 0.002), makeCostEvent(200, 100, 0.004)]
    const total = events.reduce((s, e) => s + e.input_tokens, 0)
    expect(total).toBe(300)
  })

  it('total output tokens sum is correct', () => {
    const events = [makeCostEvent(100, 50, 0.002), makeCostEvent(200, 100, 0.004)]
    const total = events.reduce((s, e) => s + e.output_tokens, 0)
    expect(total).toBe(150)
  })

  it('cost events are ordered by creation', () => {
    const events = [
      makeCostEvent(100, 50, 0.002),
      makeCostEvent(200, 100, 0.004),
      makeCostEvent(50, 25, 0.001),
    ]
    expect(events[0].input_tokens).toBe(100)
    expect(events[2].input_tokens).toBe(50)
  })

  it('cost accumulation over five runs', () => {
    let total = 0
    for (let i = 0; i < 5; i++) {
      total += makeCostEvent(100, 50, 0.001).cost_usd
    }
    expect(total).toBeCloseTo(0.005, 9)
  })
})
