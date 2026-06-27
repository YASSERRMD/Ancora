jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec, collectEvents } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

function makeHitlRuntime(
  preResumeEvents: RunEvent[],
  postResumeEvents: RunEvent[]
): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  const postQueues = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `hitl-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        ...preResumeEvents.map(e => ({ ...e, run_id: id })),
      ])
      postQueues.set(id, postResumeEvents.map(e => ({ ...e, run_id: id })))
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun(id: string): void {
      const post = postQueues.get(id) ?? []
      const q = runs.get(id) ?? []
      post.forEach(e => q.push(e))
      runs.set(id, q)
    },
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

describe('human-in-loop example smoke test', () => {
  it('collects pre-resume events before pausing', async () => {
    const rt = makeHitlRuntime(
      [{ kind: 'token', run_id: '', text: 'pre-resume' }],
      [{ kind: 'token', run_id: '', text: 'post-resume' }, { kind: 'completed', run_id: '' }],
    )
    const agent = new Agent(rt)
    const handle = agent.run(buildSpec('model'))
    const preEvents: RunEvent[] = []
    for await (const ev of handle) {
      preEvents.push(ev)
    }
    expect(preEvents.some(e => e.kind === 'token' && (e as any).text === 'pre-resume')).toBe(true)
    agent.free()
  })

  it('post-resume events are collected after resume()', async () => {
    const rt = makeHitlRuntime(
      [],
      [{ kind: 'token', run_id: '', text: 'decision-accepted' }, { kind: 'completed', run_id: '' }],
    )
    const agent = new Agent(rt)
    const handle = agent.run(buildSpec('model'))
    await collectEvents(handle)

    handle.resume('approved')
    const postEvents = await handle.run('approved')
    expect(postEvents.some(e => e.kind === 'completed')).toBe(true)
    agent.free()
  })

  it('run handle resume accepts string decision', async () => {
    const rt = makeHitlRuntime([], [{ kind: 'completed', run_id: '' }])
    const agent = new Agent(rt)
    const handle = agent.run(buildSpec('model'))
    await collectEvents(handle)
    expect(() => handle.resume('approved')).not.toThrow()
    agent.free()
  })

  it('run handle resume accepts Uint8Array decision', async () => {
    const rt = makeHitlRuntime([], [{ kind: 'completed', run_id: '' }])
    const agent = new Agent(rt)
    const handle = agent.run(buildSpec('model'))
    await collectEvents(handle)
    const decision = new TextEncoder().encode('approved')
    expect(() => handle.resume(decision)).not.toThrow()
    agent.free()
  })
})
