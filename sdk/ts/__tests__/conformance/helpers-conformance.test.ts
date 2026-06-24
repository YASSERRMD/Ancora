import { collectEvents, tokenText } from '../../helpers'
import { RunEvent } from '../../schemas'

function makeIterable(events: RunEvent[]): AsyncIterable<RunEvent> {
  return {
    async *[Symbol.asyncIterator]() {
      for (const ev of events) yield ev
    },
  }
}

describe('collectEvents conformance', () => {
  it('returns empty array for empty iterable', async () => {
    const events = await collectEvents(makeIterable([]))
    expect(events).toEqual([])
  })

  it('collects all events in order', async () => {
    const input: RunEvent[] = [
      { kind: 'started', run_id: 'r', spec: '{}' },
      { kind: 'token', run_id: 'r', text: 'a' },
      { kind: 'completed', run_id: 'r' },
    ]
    const events = await collectEvents(makeIterable(input))
    expect(events).toHaveLength(3)
    expect(events[0].kind).toBe('started')
    expect(events[2].kind).toBe('completed')
  })

  it('preserves all event fields', async () => {
    const input: RunEvent[] = [
      { kind: 'token', run_id: 'r1', text: 'hello' },
    ]
    const events = await collectEvents(makeIterable(input))
    expect((events[0] as { text: string }).text).toBe('hello')
  })
})

describe('tokenText conformance', () => {
  it('returns empty string for no events', () => {
    expect(tokenText([])).toBe('')
  })

  it('returns empty string when no token events', () => {
    expect(tokenText([
      { kind: 'started', run_id: 'r', spec: '{}' },
      { kind: 'completed', run_id: 'r' },
    ])).toBe('')
  })

  it('concatenates single token', () => {
    expect(tokenText([{ kind: 'token', run_id: 'r', text: 'hello' }])).toBe('hello')
  })

  it('concatenates tokens in order', () => {
    expect(tokenText([
      { kind: 'token', run_id: 'r', text: 'foo' },
      { kind: 'token', run_id: 'r', text: 'bar' },
      { kind: 'token', run_id: 'r', text: 'baz' },
    ])).toBe('foobarbaz')
  })

  it('ignores non-token events between tokens', () => {
    expect(tokenText([
      { kind: 'started', run_id: 'r', spec: '{}' },
      { kind: 'token', run_id: 'r', text: 'A' },
      { kind: 'token', run_id: 'r', text: 'B' },
      { kind: 'completed', run_id: 'r' },
    ])).toBe('AB')
  })
})
