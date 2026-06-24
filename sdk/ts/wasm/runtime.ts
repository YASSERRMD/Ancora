import { HttpTransport, TransportOptions } from './transport'
import { AgentSpec } from '../schemas'
import { encodeSpec, parseEvent } from '../wire'
import { RunEvent } from '../schemas'

export class WasmRunHandle {
  readonly runId: string
  private _transport: HttpTransport
  private _done = false

  constructor(runId: string, transport: HttpTransport) {
    this.runId = runId
    this._transport = transport
  }

  async *events(): AsyncGenerator<RunEvent> {
    while (!this._done) {
      const raw = await this._transport.pollRun(this.runId)
      if (raw === null) {
        this._done = true
        break
      }
      const ev = parseEvent(raw)
      yield ev
      if (ev.kind === 'completed') {
        this._done = true
        break
      }
    }
  }

  [Symbol.asyncIterator](): AsyncGenerator<RunEvent> {
    return this.events()
  }

  async resume(decision: string | Uint8Array): Promise<void> {
    const bytes =
      typeof decision === 'string'
        ? new TextEncoder().encode(decision)
        : decision
    await this._transport.resumeRun(this.runId, bytes)
  }
}

export class WasmRuntime {
  private _transport: HttpTransport

  constructor(opts: TransportOptions | HttpTransport) {
    this._transport = opts instanceof HttpTransport ? opts : new HttpTransport(opts)
  }

  get transport(): HttpTransport {
    return this._transport
  }

  async run(spec: AgentSpec): Promise<WasmRunHandle> {
    const bytes = encodeSpec(spec)
    const runId = await this._transport.startRun(bytes)
    return new WasmRunHandle(runId, this._transport)
  }
}
