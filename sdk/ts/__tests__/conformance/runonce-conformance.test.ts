jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { runOnce } from '../../helpers'
import { tokenText } from '../../helpers'
import { buildSpec } from '../../wire'

jest.mock('../../agent', () => {
  const { buildSpec: bs } = jest.requireActual('../../wire') as typeof import('../../wire')
  const { parseEvent: pe } = jest.requireActual('../../wire') as typeof import('../../wire')
  const EVENTS = [
    JSON.stringify({ kind: 'started', run_id: 'r1', spec: '{}' }),
    JSON.stringify({ kind: 'token', run_id: 'r1', text: 'ok' }),
    JSON.stringify({ kind: 'completed', run_id: 'r1' }),
  ]
  let idx = 0

  class MockRunHandle {
    readonly runId = 'r1'
    async *[Symbol.asyncIterator]() {
      while (idx < EVENTS.length) {
        yield pe(EVENTS[idx++])
      }
    }
  }

  class MockAgent {
    run() {
      idx = 0
      return new MockRunHandle()
    }
    free() {}
    get isFreed() { return false }
  }

  void bs
  return { Agent: MockAgent, RunHandle: MockRunHandle }
})

describe('runOnce conformance', () => {
  it('returns an array of RunEvents', async () => {
    const spec = buildSpec('test')
    const events = await runOnce(spec)
    expect(Array.isArray(events)).toBe(true)
    expect(events.length).toBeGreaterThan(0)
  })

  it('first event is started', async () => {
    const spec = buildSpec('test')
    const events = await runOnce(spec)
    expect(events[0].kind).toBe('started')
  })

  it('last event is completed', async () => {
    const spec = buildSpec('test')
    const events = await runOnce(spec)
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('tokenText works on runOnce output', async () => {
    const spec = buildSpec('test')
    const events = await runOnce(spec)
    expect(tokenText(events)).toBe('ok')
  })
})
