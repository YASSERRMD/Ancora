jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec, collectEvents, tokenText } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

function makeOfflineRuntime(tokens: string[]): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `offline-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        ...tokens.map(t => ({ kind: 'token' as const, run_id: id, text: t })),
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

describe('single-agent example smoke test', () => {
  it('runs without throwing', async () => {
    const spec = buildSpec('claude-3-5-sonnet', { maxTokens: 1024 })
    const rt = makeOfflineRuntime(['Hello', ' world'])
    const agent = new Agent(rt)
    const events = await collectEvents(agent.run(spec))
    expect(tokenText(events)).toBe('Hello world')
    agent.free()
  })

  it('first event is started', async () => {
    const rt = makeOfflineRuntime(['Hi'])
    const agent = new Agent(rt)
    const events = await collectEvents(agent.run(buildSpec('test')))
    expect(events[0].kind).toBe('started')
    agent.free()
  })

  it('last event is completed', async () => {
    const rt = makeOfflineRuntime(['Hi'])
    const agent = new Agent(rt)
    const events = await collectEvents(agent.run(buildSpec('test')))
    expect(events[events.length - 1].kind).toBe('completed')
    agent.free()
  })
})
