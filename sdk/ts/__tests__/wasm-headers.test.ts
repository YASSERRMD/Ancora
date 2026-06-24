import { HttpTransport } from '../wasm/transport'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

describe('HttpTransport custom headers', () => {
  it('includes custom Authorization header in startRun', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r1' }) })
    const t = new HttpTransport({
      baseUrl: 'http://localhost:8080',
      headers: { Authorization: 'Bearer token123' },
    })
    await t.startRun(new TextEncoder().encode('{}'))
    const call = mockFetch.mock.calls[0]
    expect(call[1].headers['Authorization']).toBe('Bearer token123')
  })

  it('default Content-Type is application/json', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r1' }) })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    await t.startRun(new Uint8Array())
    expect(mockFetch.mock.calls[0][1].headers['Content-Type']).toBe('application/json')
  })

  it('custom headers are merged with defaults', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r1' }) })
    const t = new HttpTransport({
      baseUrl: 'http://localhost:8080',
      headers: { 'X-Custom': 'value' },
    })
    await t.startRun(new Uint8Array())
    const headers = mockFetch.mock.calls[0][1].headers
    expect(headers['Content-Type']).toBe('application/json')
    expect(headers['X-Custom']).toBe('value')
  })
})
