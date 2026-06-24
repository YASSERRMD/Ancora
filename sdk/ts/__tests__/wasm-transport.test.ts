import { HttpTransport } from '../wasm/transport'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

describe('HttpTransport constructor', () => {
  it('stores baseUrl without trailing slash', () => {
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080/' })
    expect(t.baseUrl).toBe('http://localhost:8080')
  })

  it('works with no trailing slash', () => {
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    expect(t.baseUrl).toBe('http://localhost:8080')
  })
})

describe('HttpTransport.startRun', () => {
  it('POSTs to /v1/runs and returns run_id', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ run_id: 'run-abc' }),
    })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    const id = await t.startRun(new TextEncoder().encode('{}'))
    expect(id).toBe('run-abc')
    expect(mockFetch).toHaveBeenCalledWith(
      'http://localhost:8080/v1/runs',
      expect.objectContaining({ method: 'POST' })
    )
  })

  it('throws on non-ok response', async () => {
    mockFetch.mockResolvedValueOnce({ ok: false, status: 500 })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    await expect(t.startRun(new Uint8Array())).rejects.toThrow('500')
  })
})

describe('HttpTransport.pollRun', () => {
  it('GETs /v1/runs/{id}/events and returns Uint8Array', async () => {
    const payload = new TextEncoder().encode('{"kind":"completed","run_id":"r1"}')
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      arrayBuffer: async () => payload.buffer,
    })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    const result = await t.pollRun('r1')
    expect(result).not.toBeNull()
    expect(result!.length).toBeGreaterThan(0)
  })

  it('returns null on 204', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    expect(await t.pollRun('r1')).toBeNull()
  })

  it('returns null on empty body', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => new ArrayBuffer(0) })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    expect(await t.pollRun('r1')).toBeNull()
  })

  it('URL-encodes the run ID', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    await t.pollRun('run/with/slashes')
    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('run%2Fwith%2Fslashes'),
      expect.any(Object)
    )
  })
})

describe('HttpTransport.resumeRun', () => {
  it('POSTs to /v1/runs/{id}/resume', async () => {
    mockFetch.mockResolvedValueOnce({ ok: true })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    await t.resumeRun('r1', new TextEncoder().encode('approve'))
    expect(mockFetch).toHaveBeenCalledWith(
      'http://localhost:8080/v1/runs/r1/resume',
      expect.objectContaining({ method: 'POST' })
    )
  })

  it('throws on non-ok response', async () => {
    mockFetch.mockResolvedValueOnce({ ok: false, status: 404 })
    const t = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    await expect(t.resumeRun('r1', new Uint8Array())).rejects.toThrow('404')
  })
})
