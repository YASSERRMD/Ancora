import { parseSseLine, parseSseChunk } from '../wasm/sse'

describe('parseSseLine', () => {
  it('parses a valid data line', () => {
    const ev = parseSseLine('data: {"kind":"completed","run_id":"r1"}')
    expect(ev?.kind).toBe('completed')
  })

  it('returns null for non-data lines', () => {
    expect(parseSseLine(': keep-alive')).toBeNull()
    expect(parseSseLine('event: message')).toBeNull()
    expect(parseSseLine('')).toBeNull()
  })

  it('returns null for [DONE] sentinel', () => {
    expect(parseSseLine('data: [DONE]')).toBeNull()
  })

  it('returns null for malformed JSON', () => {
    expect(parseSseLine('data: not-json')).toBeNull()
  })

  it('parses a token event', () => {
    const ev = parseSseLine('data: {"kind":"token","run_id":"r1","text":"Hello"}')
    expect(ev?.kind).toBe('token')
    if (ev?.kind === 'token') expect(ev.text).toBe('Hello')
  })
})

describe('parseSseChunk', () => {
  it('extracts events from a multi-line chunk', () => {
    const chunk = [
      'data: {"kind":"started","run_id":"r1","spec":"{}"}',
      'data: {"kind":"token","run_id":"r1","text":"Hi"}',
      'data: {"kind":"completed","run_id":"r1"}',
    ].join('\n')
    const events = Array.from(parseSseChunk(chunk))
    expect(events).toHaveLength(3)
    expect(events[0].kind).toBe('started')
    expect(events[2].kind).toBe('completed')
  })

  it('skips non-data lines', () => {
    const chunk = [
      ': comment',
      'data: {"kind":"completed","run_id":"r1"}',
      '',
    ].join('\n')
    const events = Array.from(parseSseChunk(chunk))
    expect(events).toHaveLength(1)
  })

  it('returns empty for empty chunk', () => {
    expect(Array.from(parseSseChunk(''))).toHaveLength(0)
  })
})
