import { RunHandle } from './agent'
import { RunEvent } from './schemas'
import { ToolRegistry } from './tools'

export type ToolBridgeEvent =
  | RunEvent
  | { kind: 'tool_result'; run_id: string; name: string; result: unknown }

export class ToolBridge {
  private _registry: ToolRegistry

  constructor(registry: ToolRegistry) {
    this._registry = registry
  }

  get registry(): ToolRegistry {
    return this._registry
  }

  async *run(handle: RunHandle): AsyncGenerator<ToolBridgeEvent> {
    for await (const ev of handle) {
      if (ev.kind === 'tool_call') {
        const input = JSON.parse(ev.input) as unknown
        let result: unknown
        try {
          result = await this._registry.dispatch(ev.name, input)
        } catch (err) {
          result = { error: String(err) }
        }
        const resultJson = JSON.stringify(result)
        handle.resume(resultJson)
        yield { kind: 'tool_result', run_id: ev.run_id, name: ev.name, result }
      } else {
        yield ev
      }
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function createToolBridge(...tools: ToolRegistry extends { register(t: infer T): any } ? T[] : never[]): ToolBridge {
  const registry = new ToolRegistry()
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  tools.forEach((t) => registry.register(t as any))
  return new ToolBridge(registry)
}
