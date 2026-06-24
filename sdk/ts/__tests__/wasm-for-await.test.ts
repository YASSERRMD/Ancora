import { WasmRunHandle } from '../wasm/runtime'
import { HttpTransport } from '../wasm/transport'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

function jsonBytes(obj: object): ArrayBuffer {
  return new TextEncoder().encode(JSON.stringify(obj)).buffer as ArrayBuffer
}

describe('WasmRunHandle for-await-of', () => {
  it('collects all events via for-await-of', async () => {
    const transport = new HttpTransport({ baseUrl: 'http://host' })
    mockFetch
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBytes({ kind: 'started', run_id: 'r1', spec: '{}' }) })
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBytes({ kind: 'token', run_id: 'r1', text: 'Hi' }) })
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBytes({ kind: 'completed', run_id: 'r1' }) })

    const h = new WasmRunHandle('r1', transport)
    const kinds: string[] = []
    for await (const ev of h) {
      kinds.push(ev.kind)
    }
    expect(kinds).toEqual(['started', 'token', 'completed'])
  })

  it('stops at the first completed event', async () => {
    const transport = new HttpTransport({ baseUrl: 'http://host' })
    mockFetch
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBytes({ kind: 'completed', run_id: 'r2' }) })
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBytes({ kind: 'token', run_id: 'r2', text: 'extra' }) })

    const h = new WasmRunHandle('r2', transport)
    const kinds: string[] = []
    for await (const ev of h) {
      kinds.push(ev.kind)
    }
    expect(kinds).toEqual(['completed'])
    expect(mockFetch).toHaveBeenCalledTimes(1)
  })

  it('handles a null poll result by stopping iteration', async () => {
    const transport = new HttpTransport({ baseUrl: 'http://host' })
    mockFetch.mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })

    const h = new WasmRunHandle('r3', transport)
    const events: object[] = []
    for await (const ev of h) {
      events.push(ev)
    }
    expect(events).toHaveLength(0)
  })
})
