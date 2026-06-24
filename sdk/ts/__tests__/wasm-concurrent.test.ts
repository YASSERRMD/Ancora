import { WasmRuntime, WasmRunHandle } from '../wasm/runtime'
import { HttpTransport } from '../wasm/transport'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

function makeEventBytes(event: object): ArrayBuffer {
  return new TextEncoder().encode(JSON.stringify(event)).buffer as ArrayBuffer
}

describe('WasmRuntime concurrent runs', () => {
  it('two concurrent runs have independent run IDs', async () => {
    mockFetch
      .mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'run-A' }) })
      .mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'run-B' }) })

    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const specA = { model: 'gpt-4', instructions: 'A', tools: [], max_tokens: 10, temperature: 0.5 }
    const specB = { model: 'gpt-4', instructions: 'B', tools: [], max_tokens: 10, temperature: 0.5 }

    const [hA, hB] = await Promise.all([rt.run(specA), rt.run(specB)])
    expect(hA.runId).toBe('run-A')
    expect(hB.runId).toBe('run-B')
    expect(hA.runId).not.toBe(hB.runId)
  })

  it('each handle polls its own run ID', async () => {
    mockFetch
      .mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: 'run-C' }) })
      .mockResolvedValueOnce({
        ok: true,
        status: 200,
        arrayBuffer: async () => makeEventBytes({ kind: 'completed', run_id: 'run-C' }),
      })

    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const spec = { model: 'gpt-4', instructions: 'x', tools: [], max_tokens: 10, temperature: 0.5 }
    const h = await rt.run(spec)
    const events: string[] = []
    for await (const ev of h) {
      events.push(ev.kind)
    }
    expect(events).toContain('completed')
  })

  it('third run starts after first two complete', async () => {
    for (const id of ['run-1', 'run-2', 'run-3']) {
      mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: id }) })
    }
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const spec = { model: 'gpt-4', instructions: 'z', tools: [], max_tokens: 1, temperature: 0 }
    const handles = await Promise.all([rt.run(spec), rt.run(spec), rt.run(spec)])
    expect(handles.map(h => h.runId)).toEqual(['run-1', 'run-2', 'run-3'])
  })
})
