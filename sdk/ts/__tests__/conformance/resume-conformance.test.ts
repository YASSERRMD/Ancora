jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, RunHandle } from '../../agent'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'
import { collectEvents } from '../../helpers'
import { buildSpec } from '../../wire'

function makeResumableRuntime(): { rt: Runtime; resumed: string[] } {
  const resumed: string[] = []
  const runs = new Map<string, RunEvent[]>()
  let counter = 0

  const rt: Runtime = {
    startRun() {
      const id = `run-${++counter}`
      runs.set(id, [
        { kind: 'started', run_id: id, spec: '{}' },
        { kind: 'tool_call', run_id: id, name: 'ping', input: '{}' },
      ])
      return id
    },
    pollRun(id: string) {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun(id: string, decision: string | Uint8Array) {
      const d = typeof decision === 'string' ? decision : new TextDecoder().decode(decision)
      resumed.push(d)
      const q = runs.get(id) ?? []
      q.push({ kind: 'completed', run_id: id })
      runs.set(id, q)
    },
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime

  return { rt, resumed }
}

const SPEC = buildSpec('test')

describe('RunHandle resume conformance', () => {
  it('resume() feeds a decision and continues events', async () => {
    const { rt, resumed } = makeResumableRuntime()
    const agent = new Agent(rt)
    const handle = agent.run(SPEC)

    const events: RunEvent[] = []
    for await (const ev of handle) {
      events.push(ev)
      if (ev.kind === 'tool_call') {
        handle.resume('{"pong":true}')
      }
    }
    expect(events.some(e => e.kind === 'tool_call')).toBe(true)
    expect(events.some(e => e.kind === 'completed')).toBe(true)
    expect(resumed).toContain('{"pong":true}')
  })

  it('run() resumes and collects subsequent events', async () => {
    const { rt } = makeResumableRuntime()
    const agent = new Agent(rt)
    const handle = agent.run(SPEC)

    let seenToolCall = false
    for await (const ev of handle) {
      if (ev.kind === 'tool_call') {
        seenToolCall = true
        const rest = await handle.run('{"ok":true}')
        expect(rest.some(e => e.kind === 'completed')).toBe(true)
        break
      }
    }
    expect(seenToolCall).toBe(true)
  })
})
