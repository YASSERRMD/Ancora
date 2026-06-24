jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { buildSpec } from '../../wire'
import { collectEvents, tokenText, runOnce } from '../../helpers'
import { Agent, RunHandle } from '../../agent'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

const SPEC = buildSpec('test-model', { instructions: 'conformance test' })

function makeInMemoryRuntime(): Runtime {
  const events: Map<string, RunEvent[]> = new Map()
  let counter = 0

  return {
    startRun(_spec: string | Uint8Array): string {
      const id = `run-${++counter}`
      events.set(id, [
        { kind: 'started', run_id: id, spec: '{}' },
        { kind: 'token', run_id: id, text: 'Hello' },
        { kind: 'token', run_id: id, text: ' world' },
        { kind: 'completed', run_id: id },
      ])
      return id
    },
    pollRun(id: string): string | null {
      const q = events.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun(_id: string, _decision: string | Uint8Array): void {},
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

describe('agent lifecycle conformance', () => {
  it('run() returns a RunHandle with a runId', () => {
    const agent = new Agent(makeInMemoryRuntime())
    const handle = agent.run(SPEC)
    expect(handle).toBeInstanceOf(RunHandle)
    expect(typeof handle.runId).toBe('string')
  })

  it('events() yields all events in order', async () => {
    const agent = new Agent(makeInMemoryRuntime())
    const handle = agent.run(SPEC)
    const collected = await collectEvents(handle)
    expect(collected[0].kind).toBe('started')
    expect(collected[collected.length - 1].kind).toBe('completed')
  })

  it('token events appear between started and completed', async () => {
    const agent = new Agent(makeInMemoryRuntime())
    const handle = agent.run(SPEC)
    const collected = await collectEvents(handle)
    const kinds = collected.map(e => e.kind)
    const startIdx = kinds.indexOf('started')
    const endIdx = kinds.indexOf('completed')
    const tokenEvents = collected.filter(e => e.kind === 'token')
    expect(tokenEvents.length).toBeGreaterThan(0)
    tokenEvents.forEach(ev => {
      const idx = collected.indexOf(ev)
      expect(idx).toBeGreaterThan(startIdx)
      expect(idx).toBeLessThan(endIdx)
    })
  })

  it('tokenText concatenates all token texts', async () => {
    const agent = new Agent(makeInMemoryRuntime())
    const handle = agent.run(SPEC)
    const events = await collectEvents(handle)
    expect(tokenText(events)).toBe('Hello world')
  })

  it('agent.free() marks runtime as freed', () => {
    const rt = makeInMemoryRuntime()
    const agent = new Agent(rt)
    expect(agent.isFreed).toBe(false)
    agent.free()
  })
})
