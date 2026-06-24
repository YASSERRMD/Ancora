import { parseEvent } from '../../wire'
import { tokenText, collectEvents } from '../../helpers'
import { RunEvent } from '../../schemas'
import FIXTURES from './fixtures.json'

function makeIterable(events: RunEvent[]): AsyncIterable<RunEvent> {
  return {
    async *[Symbol.asyncIterator]() {
      for (const ev of events) yield ev
    },
  }
}

describe('event ordering conformance', () => {
  it('fixture events are in the expected sequence', () => {
    const kinds = FIXTURES.events.map(e => e.kind)
    expect(kinds[0]).toBe('started')
    expect(kinds[kinds.length - 1]).toBe('completed')
    const tokenIdx = kinds.findIndex(k => k === 'token')
    const completedIdx = kinds.findIndex(k => k === 'completed')
    expect(tokenIdx).toBeGreaterThan(0)
    expect(tokenIdx).toBeLessThan(completedIdx)
  })

  it('fixture has exactly 3 token events', () => {
    const tokens = FIXTURES.events.filter(e => e.kind === 'token')
    expect(tokens).toHaveLength(3)
  })

  it('token text from fixture events is Hello, world!', async () => {
    const events = FIXTURES.events.map(ev => parseEvent(JSON.stringify(ev)))
    const all = await collectEvents(makeIterable(events))
    expect(tokenText(all)).toBe('Hello, world!')
  })

  it('fixture run_id is consistent across all events', () => {
    const runIds = FIXTURES.events.map(e => e.run_id)
    expect(new Set(runIds).size).toBe(1)
    expect(runIds[0]).toBe('fixture-run-1')
  })

  it('tool_call event run_id matches resumed event run_id', () => {
    const toolCallId = FIXTURES.tool_call_events[0].run_id
    const resumedId = FIXTURES.resumed_events[0].run_id
    expect(toolCallId).toBe(resumedId)
  })

  it('fixture events parse to correct run_id values', () => {
    for (const ev of FIXTURES.events) {
      const parsed = parseEvent(JSON.stringify(ev))
      expect(parsed.run_id).toBe('fixture-run-1')
    }
  })
})
