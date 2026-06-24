import { WasmRuntime } from '../wasm/runtime'
import { AgentSpecSchema } from '../schemas'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

const SPEC = AgentSpecSchema.parse({ model: 'test' })

function makeEventBuffer(ev: object): ArrayBuffer {
  const bytes = new TextEncoder().encode(JSON.stringify(ev))
  return bytes.buffer as ArrayBuffer
}

describe('WasmRunHandle.resume', () => {
  it('calls resumeRun on transport', async () => {
    mockFetch
      .mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r1' }) })
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => makeEventBuffer({ kind: 'started', run_id: 'r1', spec: '{}' }) })
      .mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
      .mockResolvedValueOnce({ ok: true })
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const handle = await rt.run(SPEC)
    for await (const _ of handle) {}
    await handle.resume('approve')
    const resumeCall = mockFetch.mock.calls.find(
      (c) => typeof c[0] === 'string' && c[0].includes('/resume')
    )
    expect(resumeCall).toBeDefined()
  })

  it('resume accepts string decision', async () => {
    mockFetch
      .mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r2' }) })
      .mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
      .mockResolvedValueOnce({ ok: true })
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const handle = await rt.run(SPEC)
    await expect(handle.resume('go')).resolves.toBeUndefined()
  })

  it('resume accepts Uint8Array decision', async () => {
    mockFetch
      .mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'r3' }) })
      .mockResolvedValueOnce({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
      .mockResolvedValueOnce({ ok: true })
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const handle = await rt.run(SPEC)
    const bytes = new TextEncoder().encode('go')
    await expect(handle.resume(bytes)).resolves.toBeUndefined()
  })
})
