jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

function makeStreamRuntime(tokens: string[]): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `stream-${++counter}`
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

describe('streaming chat example smoke test', () => {
  it('streams tokens in order via for-await', async () => {
    const tokens = ['Hello', '!', ' How', ' can', ' I', ' help', '?']
    const rt = makeStreamRuntime(tokens)
    const agent = new Agent(rt)
    const handle = agent.run(buildSpec('claude', { maxTokens: 2048 }))

    const received: string[] = []
    for await (const ev of handle) {
      if (ev.kind === 'token') received.push(ev.text)
    }
    expect(received).toEqual(tokens)
    agent.free()
  })

  it('completes after all tokens are yielded', async () => {
    const rt = makeStreamRuntime(['A', 'B', 'C'])
    const agent = new Agent(rt)
    let completed = false
    for await (const ev of agent.run(buildSpec('test'))) {
      if (ev.kind === 'completed') completed = true
    }
    expect(completed).toBe(true)
    agent.free()
  })

  it('processes 100 tokens without error', async () => {
    const tokens = Array.from({ length: 100 }, (_, i) => `word${i}`)
    const rt = makeStreamRuntime(tokens)
    const agent = new Agent(rt)
    let count = 0
    for await (const ev of agent.run(buildSpec('test'))) {
      if (ev.kind === 'token') count++
    }
    expect(count).toBe(100)
    agent.free()
  })
})
