let nativeBinding: NativeModule | null = null

interface NativeModule {
  Runtime: new () => NativeRuntimeInstance
  version: () => string
}

interface NativeRuntimeInstance {
  readonly isFreed: boolean
  free(): void
  startRun(specBytes: Buffer): string
  pollRun(runId: string): Buffer | null
  resumeRun(runId: string, decision: Buffer): void
}

function loadNativeModule(): NativeModule {
  if (nativeBinding) return nativeBinding
  try {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    nativeBinding = require('./ancora.node') as NativeModule
    return nativeBinding
  } catch {
    throw new Error(
      'ancora native module not found. Build it first with: npm run build'
    )
  }
}

export class Runtime {
  private _inner: NativeRuntimeInstance

  constructor() {
    const mod = loadNativeModule()
    this._inner = new mod.Runtime()
  }

  get isFreed(): boolean {
    return this._inner.isFreed
  }

  free(): void {
    this._inner.free()
  }

  startRun(spec: string | Uint8Array): string {
    const bytes =
      typeof spec === 'string' ? Buffer.from(spec, 'utf8') : Buffer.from(spec)
    return this._inner.startRun(bytes)
  }

  pollRun(runId: string): string | null {
    const raw = this._inner.pollRun(runId)
    if (raw === null || raw === undefined) return null
    return raw.toString('utf8')
  }

  resumeRun(runId: string, decision: string | Uint8Array): void {
    const bytes =
      typeof decision === 'string'
        ? Buffer.from(decision, 'utf8')
        : Buffer.from(decision)
    this._inner.resumeRun(runId, bytes)
  }
}

export function version(): string {
  return loadNativeModule().version()
}

export {
  ToolInputPropertySchema,
  ToolInputSchemaSchema,
  ToolSpecSchema,
  AgentSpecSchema,
  RunEventSchema,
  ToolCallSchema,
} from './schemas'
export type { ToolSpec, AgentSpec, RunEvent, ToolCall } from './schemas'
export { encodeSpec, decodeSpec, parseEvent, validateSpec, buildSpec } from './wire'
export { Agent, RunHandle } from './agent'
