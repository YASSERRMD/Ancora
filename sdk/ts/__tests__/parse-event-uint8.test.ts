import { parseEvent } from '../wire'

describe('parseEvent with Uint8Array input', () => {
  it('parses a Uint8Array containing a completed event', () => {
    const bytes = new TextEncoder().encode('{"kind":"completed","run_id":"r1"}')
    const ev = parseEvent(bytes)
    expect(ev.kind).toBe('completed')
  })

  it('parses a Uint8Array containing a token event', () => {
    const bytes = new TextEncoder().encode('{"kind":"token","run_id":"r1","text":"x"}')
    const ev = parseEvent(bytes)
    expect(ev.kind).toBe('token')
    if (ev.kind === 'token') expect(ev.text).toBe('x')
  })

  it('parses a Buffer input (Node.js)', () => {
    const buf = Buffer.from('{"kind":"completed","run_id":"r2"}', 'utf8')
    const ev = parseEvent(buf)
    expect(ev.kind).toBe('completed')
  })

  it('parses a string input', () => {
    const ev = parseEvent('{"kind":"completed","run_id":"r3"}')
    expect(ev.kind).toBe('completed')
  })

  it('multi-byte UTF-8 content in Uint8Array', () => {
    const bytes = new TextEncoder().encode('{"kind":"token","run_id":"r1","text":"éàü"}')
    const ev = parseEvent(bytes)
    expect(ev.kind).toBe('token')
    if (ev.kind === 'token') expect(ev.text).toBe('éàü')
  })
})
