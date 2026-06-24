jest.mock('../ancora.node', () => {
  const runs: Record<string, string[]> = {}
  let counter = 0
  return {
    Runtime: class MockRuntime {
      private _freed = false
      get isFreed(): boolean { return this._freed }
      free(): void { this._freed = true }
      startRun(specBytes: Buffer): string {
        const id = `run-${counter++}`
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec: specBytes.toString('utf8') }),
          JSON.stringify({ kind: 'completed', run_id: id }),
        ]
        return id
      }
      pollRun(runId: string): Buffer | null {
        const q = runs[runId]
        if (!q || q.length === 0) return null
        return Buffer.from(q.shift()!, 'utf8')
      }
      resumeRun(): void {}
    },
    version: () => '0.1.0',
  }
}, { virtual: true })

import { Agent } from '../agent'
import { AgentSpecSchema } from '../schemas'

async function collectEvents(gen: AsyncIterable<{ kind: string }>) {
  const events = []
  for await (const ev of gen) events.push(ev)
  return events
}

describe('multiple concurrent runs', () => {
  it('two handles have different run IDs', () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'gpt-4' })
    const h1 = agent.run(spec)
    const h2 = agent.run(spec)
    expect(h1.runId).not.toBe(h2.runId)
  })

  it('draining one handle does not affect another', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'gpt-4' })
    const h1 = agent.run(spec)
    const h2 = agent.run(spec)
    await collectEvents(h1)
    const evs2 = await collectEvents(h2)
    expect(evs2.length).toBeGreaterThan(0)
    expect(evs2[evs2.length - 1].kind).toBe('completed')
  })

  it('parallel drain with Promise.all', async () => {
    const agent = new Agent()
    const spec = AgentSpecSchema.parse({ model: 'test' })
    const [evs1, evs2] = await Promise.all([
      collectEvents(agent.run(spec)),
      collectEvents(agent.run(spec)),
    ])
    expect(evs1[evs1.length - 1].kind).toBe('completed')
    expect(evs2[evs2.length - 1].kind).toBe('completed')
  })
})
