import { Runtime } from 'ancora'

type RunEvent =
  | { kind: 'started'; run_id: string; spec: string }
  | { kind: 'token'; run_id: string; text: string }
  | { kind: 'completed'; run_id: string }
  | { kind: 'tool_call'; run_id: string; name: string; input: string }

let counter = 0

const runs = new Map<string, RunEvent[]>()

export function makeOfflineRuntime(responseTokens: string[] = ['Hello', ' from', ' Ancora', '!']): Runtime {
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `offline-run-${++counter}`
      const specStr = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [
        { kind: 'started', run_id: id, spec: specStr },
        ...responseTokens.map(text => ({ kind: 'token' as const, run_id: id, text })),
        { kind: 'completed', run_id: id },
      ])
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun(id: string, decision: string | Uint8Array): void {
      const d = typeof decision === 'string' ? decision : new TextDecoder().decode(decision)
      const q = runs.get(id) ?? []
      q.push({ kind: 'completed', run_id: id })
      runs.set(id, q)
      void d
    },
    free(): void {},
    get isFreed(): boolean { return false },
  } as unknown as Runtime
}

export function makeOfflineRuntimeWithTool(
  toolName: string,
  toolInput: Record<string, unknown>,
  responseTokens: string[] = ['Done.']
): Runtime {
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `offline-run-${++counter}`
      const specStr = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [
        { kind: 'started', run_id: id, spec: specStr },
        { kind: 'tool_call', run_id: id, name: toolName, input: JSON.stringify(toolInput) },
      ])
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun(id: string, _decision: string | Uint8Array): void {
      const q = runs.get(id) ?? []
      responseTokens.forEach(text => q.push({ kind: 'token', run_id: id, text }))
      q.push({ kind: 'completed', run_id: id })
      runs.set(id, q)
    },
    free(): void {},
    get isFreed(): boolean { return false },
  } as unknown as Runtime
}
