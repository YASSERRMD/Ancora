import { Runtime } from './index'
import { AgentSpec } from './schemas'
import { encodeSpec, parseEvent } from './wire'
import { RunEvent } from './schemas'

export class RunHandle {
  readonly runId: string
  private _rt: Runtime

  constructor(runId: string, rt: Runtime) {
    this.runId = runId
    this._rt = rt
  }

  async *events(): AsyncGenerator<RunEvent> {
    let raw = this._rt.pollRun(this.runId)
    while (raw !== null) {
      yield parseEvent(raw)
      raw = this._rt.pollRun(this.runId)
    }
  }

  [Symbol.asyncIterator](): AsyncGenerator<RunEvent> {
    return this.events()
  }

  resume(decision: string | Uint8Array): void {
    this._rt.resumeRun(this.runId, decision)
  }

  async run(decision: string | Uint8Array): Promise<RunEvent[]> {
    this.resume(decision)
    const collected: RunEvent[] = []
    for await (const ev of this) {
      collected.push(ev)
    }
    return collected
  }
}

export class Agent {
  private _rt: Runtime

  constructor(rt?: Runtime) {
    this._rt = rt ?? new Runtime()
  }

  run(spec: AgentSpec): RunHandle {
    const bytes = encodeSpec(spec)
    const runId = this._rt.startRun(bytes)
    return new RunHandle(runId, this._rt)
  }

  free(): void {
    this._rt.free()
  }

  get isFreed(): boolean {
    return this._rt.isFreed
  }
}
