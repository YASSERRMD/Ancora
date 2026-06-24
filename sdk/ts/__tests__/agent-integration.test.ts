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
        const spec = specBytes.toString('utf8')
        runs[id] = [
          JSON.stringify({ kind: 'started', run_id: id, spec }),
          JSON.stringify({ kind: 'token', run_id: id, text: 'Hello' }),
          JSON.stringify({ kind: 'token', run_id: id, text: ' world' }),
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
import { buildSpec, validateSpec } from '../wire'
import { collectEvents, tokenText, runOnce } from '../helpers'

describe('full API integration', () => {
  it('validate -> build -> run -> collect -> tokenText pipeline', async () => {
    const specResult = validateSpec({ model: 'gpt-4', instructions: 'hi' })
    expect(specResult.ok).toBe(true)
    if (!specResult.ok) return

    const spec = buildSpec(specResult.spec.model, { instructions: specResult.spec.instructions })
    const events = await runOnce(spec)
    const text = tokenText(events)
    expect(text).toBe('Hello world')
  })

  it('Agent.run results match runOnce results in structure', async () => {
    const spec = buildSpec('test')
    const agent = new Agent()
    const agentEvents = await collectEvents(agent.run(spec))
    agent.free()

    const onceEvents = await runOnce(buildSpec('test'))
    expect(agentEvents.map((e) => e.kind)).toEqual(onceEvents.map((e) => e.kind))
  })
})
