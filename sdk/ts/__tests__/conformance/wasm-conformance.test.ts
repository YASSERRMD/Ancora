import { WasmRuntime, WasmRunHandle } from '../../wasm/runtime'
import { HttpTransport } from '../../wasm/transport'
import { parseSseLine, parseSseChunk } from '../../wasm/sse'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

function jsonBuf(obj: object): ArrayBuffer {
  return new TextEncoder().encode(JSON.stringify(obj)).buffer as ArrayBuffer
}

describe('WasmRuntime conformance', () => {
  it('constructs with baseUrl', () => {
    const rt = new WasmRuntime({ baseUrl: 'http://localhost' })
    expect(rt.transport).toBeInstanceOf(HttpTransport)
  })

  it('run() calls POST /v1/runs', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r1' }) })
    const rt = new WasmRuntime({ baseUrl: 'http://localhost' })
    await rt.run({ model: 'x', instructions: '', tools: [], max_tokens: 10, temperature: 0 })
    expect(mockFetch.mock.calls[0][0]).toMatch('/v1/runs')
  })

  it('run() returns WasmRunHandle with correct runId', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'run-xyz' }) })
    const rt = new WasmRuntime({ baseUrl: 'http://localhost' })
    const h = await rt.run({ model: 'x', instructions: '', tools: [], max_tokens: 10, temperature: 0 })
    expect(h).toBeInstanceOf(WasmRunHandle)
    expect(h.runId).toBe('run-xyz')
  })

  it('WasmRunHandle events() stops at null poll', async () => {
    const transport = new HttpTransport({ baseUrl: 'http://localhost' })
    mockFetch.mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
    const h = new WasmRunHandle('r1', transport)
    const events = []
    for await (const ev of h) events.push(ev)
    expect(events).toHaveLength(0)
  })

  it('WasmRunHandle collects started and completed', async () => {
    const transport = new HttpTransport({ baseUrl: 'http://localhost' })
    mockFetch
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBuf({ kind: 'started', run_id: 'r2', spec: '{}' }) })
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBuf({ kind: 'completed', run_id: 'r2' }) })
    const h = new WasmRunHandle('r2', transport)
    const kinds: string[] = []
    for await (const ev of h) kinds.push(ev.kind)
    expect(kinds).toEqual(['started', 'completed'])
  })
})

describe('SSE conformance', () => {
  it('parseSseLine parses completed event', () => {
    const ev = parseSseLine('data: {"kind":"completed","run_id":"r"}')
    expect(ev?.kind).toBe('completed')
  })

  it('parseSseLine returns null for empty line', () => {
    expect(parseSseLine('')).toBeNull()
  })

  it('parseSseLine returns null for [DONE]', () => {
    expect(parseSseLine('data: [DONE]')).toBeNull()
  })

  it('parseSseChunk returns all valid events', () => {
    const chunk = 'data: {"kind":"token","run_id":"r","text":"a"}\ndata: {"kind":"completed","run_id":"r"}'
    const events = Array.from(parseSseChunk(chunk))
    expect(events).toHaveLength(2)
    expect(events[1].kind).toBe('completed')
  })
})
