import { HttpTransport } from '../../wasm/transport'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

describe('HttpTransport conformance', () => {
  it('strips trailing slash from baseUrl', () => {
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080/' })
    expect(t.baseUrl).toBe('http://localhost:8080')
  })

  it('preserves baseUrl without trailing slash', () => {
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    expect(t.baseUrl).toBe('http://localhost:8080')
  })

  it('startRun sends POST to /v1/runs', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r1' }) })
    const t = new HttpTransport({ baseUrl: 'http://host' })
    await t.startRun(new Uint8Array())
    expect(mockFetch.mock.calls[0][0]).toBe('http://host/v1/runs')
    expect(mockFetch.mock.calls[0][1].method).toBe('POST')
  })

  it('startRun returns run_id from response', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'my-run' }) })
    const t = new HttpTransport({ baseUrl: 'http://host' })
    const id = await t.startRun(new Uint8Array())
    expect(id).toBe('my-run')
  })

  it('pollRun sends GET to /v1/runs/{runId}/events', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => new TextEncoder().encode('{"kind":"completed","run_id":"r1"}').buffer })
    const t = new HttpTransport({ baseUrl: 'http://host' })
    await t.pollRun('r1')
    expect(mockFetch.mock.calls[0][0]).toContain('/v1/runs/r1/events')
    expect(mockFetch.mock.calls[0][1].method).toBe('GET')
  })

  it('pollRun returns null on 204', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
    const t = new HttpTransport({ baseUrl: 'http://host' })
    expect(await t.pollRun('r1')).toBeNull()
  })

  it('resumeRun sends POST to /v1/runs/{runId}/resume', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true })
    const t = new HttpTransport({ baseUrl: 'http://host' })
    await t.resumeRun('r1', new Uint8Array())
    expect(mockFetch.mock.calls[0][0]).toContain('/v1/runs/r1/resume')
    expect(mockFetch.mock.calls[0][1].method).toBe('POST')
  })

  it('pollRun URL-encodes special characters in runId', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
    const t = new HttpTransport({ baseUrl: 'http://host' })
    await t.pollRun('run/with spaces')
    const url = mockFetch.mock.calls[0][0] as string
    expect(url).toContain('run%2Fwith%20spaces')
  })
})
