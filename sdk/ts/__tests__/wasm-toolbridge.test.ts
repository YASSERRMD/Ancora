import { WasmRunHandle } from '../wasm/runtime'
import { HttpTransport } from '../wasm/transport'
import { ToolRegistry, defineTool } from '../tools'
import { ToolBridge } from '../tool-bridge'
import { z } from 'zod'

const mockFetch = jest.fn()
global.fetch = mockFetch

afterEach(() => mockFetch.mockReset())

function jsonBuf(obj: object): ArrayBuffer {
  return new TextEncoder().encode(JSON.stringify(obj)).buffer as ArrayBuffer
}

describe('WasmRuntime with ToolBridge integration', () => {
  it('bridge dispatches tool_call from WasmRunHandle and resumes', async () => {
    const addTool = defineTool({
      name: 'add',
      description: 'add two numbers',
      schema: z.object({ a: z.number(), b: z.number() }),
      handler: ({ a, b }) => ({ sum: a + b }),
    })
    const registry = new ToolRegistry()
    registry.register(addTool)
    const bridge = new ToolBridge(registry)

    const transport = new HttpTransport({ baseUrl: 'http://host' })
    mockFetch
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBuf({ kind: 'tool_call', run_id: 'r1', name: 'add', input: JSON.stringify({ a: 2, b: 3 }) }) })
      .mockResolvedValueOnce({ ok: true })
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBuf({ kind: 'completed', run_id: 'r1' }) })

    const handle = new WasmRunHandle('r1', transport)
    const events: string[] = []
    for await (const ev of bridge.run(handle)) {
      events.push(ev.kind)
    }
    expect(events).not.toContain('tool_call')
    expect(events).toContain('tool_result')
    expect(events).toContain('completed')
  })

  it('bridge yields tool_result with dispatched output', async () => {
    const echoTool = defineTool({
      name: 'echo',
      description: 'echo input',
      schema: z.object({ msg: z.string() }),
      handler: ({ msg }) => msg,
    })
    const bridge = new ToolBridge(new ToolRegistry().register(echoTool) as ToolRegistry)

    const transport = new HttpTransport({ baseUrl: 'http://host' })
    mockFetch
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBuf({ kind: 'tool_call', run_id: 'r2', name: 'echo', input: JSON.stringify({ msg: 'hello' }) }) })
      .mockResolvedValueOnce({ ok: true })
      .mockResolvedValueOnce({ ok: true, status: 200, arrayBuffer: async () => jsonBuf({ kind: 'completed', run_id: 'r2' }) })

    const handle = new WasmRunHandle('r2', transport)
    const toolResults: string[] = []
    for await (const ev of bridge.run(handle)) {
      if (ev.kind === 'tool_result') toolResults.push(ev.name)
    }
    expect(toolResults).toEqual(['echo'])
  })
})
