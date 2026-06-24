import { WasmRuntime, WasmRunHandle } from '../wasm/runtime'
import { HttpTransport } from '../wasm/transport'
import { AgentSpecSchema } from '../schemas'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

const SPEC = AgentSpecSchema.parse({ model: 'test' })

function makeEventBuffer(ev: object): ArrayBuffer {
  const bytes = new TextEncoder().encode(JSON.stringify(ev))
  return bytes.buffer as ArrayBuffer
}

function mockSidecar(runId: string, events: object[]): void {
  mockFetch
    .mockResolvedValueOnce({ ok: true, json: async () => ({ run_id: runId }) })
  let i = 0
  for (const ev of events) {
    mockFetch.mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => makeEventBuffer(ev) })
    i++
  }
  mockFetch.mockResolvedValue({ ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) })
  void i
}

describe('WasmRuntime', () => {
  it('constructs with TransportOptions', () => {
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    expect(rt).toBeDefined()
    expect(rt.transport).toBeInstanceOf(HttpTransport)
  })

  it('constructs with an HttpTransport instance', () => {
    const transport = new HttpTransport({ baseUrl: 'http://localhost:8080' })
    const rt = new WasmRuntime(transport)
    expect(rt.transport).toBe(transport)
  })

  it('run() returns a WasmRunHandle with a runId', async () => {
    mockSidecar('run-1', [{ kind: 'completed', run_id: 'run-1' }])
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const handle = await rt.run(SPEC)
    expect(handle).toBeInstanceOf(WasmRunHandle)
    expect(handle.runId).toBe('run-1')
  })
})

describe('WasmRunHandle', () => {
  it('events() yields parsed RunEvents', async () => {
    mockSidecar('r1', [
      { kind: 'started', run_id: 'r1', spec: '{}' },
      { kind: 'token', run_id: 'r1', text: 'hi' },
      { kind: 'completed', run_id: 'r1' },
    ])
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const handle = await rt.run(SPEC)
    const events = []
    for await (const ev of handle.events()) {
      events.push(ev)
    }
    expect(events[0].kind).toBe('started')
    expect(events[events.length - 1].kind).toBe('completed')
  })

  it('stops iterating after completed event', async () => {
    mockSidecar('r2', [
      { kind: 'started', run_id: 'r2', spec: '{}' },
      { kind: 'completed', run_id: 'r2' },
    ])
    const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
    const handle = await rt.run(SPEC)
    const events = []
    for await (const ev of handle) {
      events.push(ev)
    }
    expect(events).toHaveLength(2)
    expect(events[1].kind).toBe('completed')
  })
})
