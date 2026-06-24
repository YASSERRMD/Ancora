import { parseEvent } from '../../wire'
import { tokenText } from '../../helpers'
import FIXTURES from './fixtures.json'

describe('journal matches the core fixture', () => {
  it('fixture spec has required fields', () => {
    expect(typeof FIXTURES.spec.model).toBe('string')
    expect(typeof FIXTURES.spec.instructions).toBe('string')
    expect(Array.isArray(FIXTURES.spec.tools)).toBe(true)
    expect(typeof FIXTURES.spec.max_tokens).toBe('number')
    expect(typeof FIXTURES.spec.temperature).toBe('number')
  })

  it('all fixture events parse without error', () => {
    for (const ev of FIXTURES.events) {
      expect(() => parseEvent(JSON.stringify(ev))).not.toThrow()
    }
  })

  it('fixture started event is first', () => {
    const first = parseEvent(JSON.stringify(FIXTURES.events[0]))
    expect(first.kind).toBe('started')
  })

  it('fixture completed event is last', () => {
    const last = parseEvent(JSON.stringify(FIXTURES.events[FIXTURES.events.length - 1]))
    expect(last.kind).toBe('completed')
  })

  it('fixture token events yield expected text', () => {
    const events = FIXTURES.events.map(ev => parseEvent(JSON.stringify(ev)))
    expect(tokenText(events)).toBe('Hello, world!')
  })

  it('all fixture events share the same run_id', () => {
    const runIds = new Set(FIXTURES.events.map(ev => ev.run_id))
    expect(runIds.size).toBe(1)
  })

  it('fixture tool_call event has name and input', () => {
    const ev = parseEvent(JSON.stringify(FIXTURES.tool_call_events[0]))
    expect(ev.kind).toBe('tool_call')
    if (ev.kind === 'tool_call') {
      expect(ev.name).toBe('get_weather')
      expect(typeof ev.input).toBe('string')
    }
  })

  it('fixture resumed event has decision', () => {
    const ev = parseEvent(JSON.stringify(FIXTURES.resumed_events[0]))
    expect(ev.kind).toBe('resumed')
    if (ev.kind === 'resumed') {
      expect(typeof ev.decision).toBe('string')
    }
  })

  it('fixture tool_call input is valid JSON', () => {
    const ev = parseEvent(JSON.stringify(FIXTURES.tool_call_events[0]))
    if (ev.kind === 'tool_call') {
      expect(() => JSON.parse(ev.input)).not.toThrow()
      const parsed = JSON.parse(ev.input) as Record<string, unknown>
      expect(parsed['city']).toBe('Paris')
    }
  })
})
