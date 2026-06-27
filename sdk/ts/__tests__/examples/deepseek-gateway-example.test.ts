jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec, collectEvents, tokenText } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

const DEEPSEEK_MODELS = [
  'deepseek-chat',
  'deepseek-coder',
  'deepseek-reasoner',
]

function makeDeepseekRuntime(): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `ds-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      const parsed = JSON.parse(s)
      const model = parsed?.model ?? 'unknown'
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        { kind: 'token', run_id: id, text: `response from ${model}` },
        { kind: 'completed', run_id: id },
      ])
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun() {},
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

describe('deepseek-gateway example smoke test', () => {
  it('runs deepseek-chat model without error', async () => {
    const rt = makeDeepseekRuntime()
    const agent = new Agent(rt)
    const spec = buildSpec('deepseek-chat')
    const events = await collectEvents(agent.run(spec))
    expect(events.some(e => e.kind === 'started')).toBe(true)
    expect(events.some(e => e.kind === 'completed')).toBe(true)
    agent.free()
  })

  it('runs all deepseek model variants sequentially', async () => {
    const rt = makeDeepseekRuntime()
    for (const model of DEEPSEEK_MODELS) {
      const agent = new Agent(rt)
      const events = await collectEvents(agent.run(buildSpec(model)))
      expect(events.some(e => e.kind === 'completed')).toBe(true)
      agent.free()
    }
  })

  it('token text contains model name when echoed', async () => {
    const rt = makeDeepseekRuntime()
    const agent = new Agent(rt)
    const events = await collectEvents(agent.run(buildSpec('deepseek-chat')))
    const text = tokenText(events)
    expect(text).toContain('deepseek-chat')
    agent.free()
  })

  it('DEEPSEEK_MODELS list has at least 2 variants', () => {
    expect(DEEPSEEK_MODELS.length).toBeGreaterThanOrEqual(2)
  })

  it('each model variant produces a distinct run ID', async () => {
    const rt = makeDeepseekRuntime()
    const runIds = new Set<string>()
    for (const model of DEEPSEEK_MODELS) {
      const agent = new Agent(rt)
      const handle = agent.run(buildSpec(model))
      runIds.add(handle.runId)
      await collectEvents(handle)
      agent.free()
    }
    expect(runIds.size).toBe(DEEPSEEK_MODELS.length)
  })
})
