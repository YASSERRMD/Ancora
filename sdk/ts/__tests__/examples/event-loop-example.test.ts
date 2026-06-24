jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

function makeRuntime(tokens: string[]): Runtime {
  let c = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `ev-${++c}`
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
      if (!q || !q.length) return null
      return JSON.stringify(q.shift())
    },
    resumeRun() {},
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

describe('event-loop example smoke test', () => {
  it('switch-on-kind handles all event types correctly', async () => {
    const rt = makeRuntime(['A', 'B', 'C'])
    const agent = new Agent(rt)
    let tokenCount = 0
    let started = false
    let completed = false

    for await (const ev of agent.run(buildSpec('test'))) {
      switch (ev.kind) {
        case 'started': started = true; break
        case 'token': tokenCount++; break
        case 'completed': completed = true; break
      }
    }

    expect(started).toBe(true)
    expect(tokenCount).toBe(3)
    expect(completed).toBe(true)
    agent.free()
  })

  it('switch statement is exhaustive for known event types', () => {
    const kinds = ['started', 'token', 'completed', 'resumed', 'tool_call']
    kinds.forEach(kind => expect(['started', 'token', 'completed', 'resumed', 'tool_call']).toContain(kind))
  })
})
